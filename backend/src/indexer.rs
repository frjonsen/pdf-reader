use std::path::PathBuf;

use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    doc,
    query::{
        BooleanQuery, ConstScoreQuery, EnableScoring, FuzzyTermQuery, Occur, Query, QueryClone,
        QueryParser, RegexQuery, TermQuery,
    },
    schema::{self, Facet, FacetOptions, Field, Schema, STORED, TEXT},
    Index, IndexReader, IndexWriter, ReloadPolicy, SnippetGenerator, Term,
};
use tokio::sync::{Mutex, MutexGuard};
use uuid::Uuid;

struct IndexFields {
    body: Field,
    document_id: Field,
    page: Field,
}

pub struct Indexer {
    index: Index,
    fields: IndexFields,
    reader: IndexReader,
    schema: Schema,
    writer: Mutex<IndexWriter>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub score: f32,
    pub page: u64,
    pub snippet: Option<String>,
}

pub struct Writer<'a> {
    fields: &'a IndexFields,
    writer: MutexGuard<'a, IndexWriter>,
    reader: &'a IndexReader,
}

impl<'a> Writer<'a> {
    pub fn index_page(
        &mut self,
        document_id: &Uuid,
        page: u64,
        contents: &str,
    ) -> Result<(), IndexerError> {
        let mut binding = Uuid::encode_buffer();
        let doc_id = document_id.as_simple().encode_lower(&mut binding);
        self.writer.add_document(doc!(
            self.fields.document_id => Facet::from(&format!("/documents/{doc_id}")),
            self.fields.page => page,
            self.fields.body => contents
        ))?;

        Ok(())
    }

    pub fn commit(mut self) -> Result<(), IndexerError> {
        self.writer.commit()?;
        self.reader.reload()?;

        Ok(())
    }
}

impl Indexer {
    const DOCUMENT_FIELD_NAME: &str = "document";
    const BODY_FIELD_NAME: &str = "body";
    const PAGE_FIELD_NAME: &str = "page";

    fn build_schema() -> Schema {
        let mut schema_builder = Schema::builder();
        schema_builder.add_facet_field(Indexer::DOCUMENT_FIELD_NAME, FacetOptions::default());
        schema_builder.add_text_field(Indexer::BODY_FIELD_NAME, TEXT | STORED);
        schema_builder.add_u64_field(Indexer::PAGE_FIELD_NAME, STORED);

        schema_builder.build()
    }

    pub fn new(index_directory: PathBuf) -> Result<Self, IndexerError> {
        log::info!("Setting up Indexer");
        if !index_directory.exists() {
            log::debug!(
                "Index directory did not exist. Creating it now at {}",
                index_directory.to_string_lossy()
            );
            std::fs::create_dir(&index_directory)?;
        }
        log::debug!(
            "Opening index directory {}",
            index_directory.to_string_lossy()
        );
        let directory =
            MmapDirectory::open(index_directory).expect("Failed to open index directory");
        log::debug!("Build schema");
        let schema = Indexer::build_schema();
        log::debug!("Opening index with schema");
        let index = Index::open_or_create(directory, schema.clone())?;
        log::debug!("Creating index reader");
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()?;
        log::debug!("Index setup complete.");
        Ok(Indexer {
            writer: Mutex::new(index.writer(50_000_000)?),
            reader,
            fields: IndexFields {
                body: schema
                    .get_field(Indexer::BODY_FIELD_NAME)
                    .expect("No body field"),
                page: schema
                    .get_field(Indexer::PAGE_FIELD_NAME)
                    .expect("No page field"),
                document_id: schema
                    .get_field(Indexer::DOCUMENT_FIELD_NAME)
                    .expect("No document field"),
            },
            schema,
            index,
        })
    }

    pub async fn get_writer(&self) -> Result<Writer, IndexerError> {
        let writer = self.writer.lock().await;
        Ok(Writer {
            reader: &self.reader,
            fields: &self.fields,
            writer,
        })
    }

    pub fn search_document(
        &self,
        doc_id: &Uuid,
        query: &str,
    ) -> Result<Vec<SearchResult>, IndexerError> {
        let mut binding = Uuid::encode_buffer();
        let doc_id = doc_id.as_simple().encode_lower(&mut binding);
        let facet_term = Term::from_facet(
            self.fields.document_id,
            &Facet::from(&format!("/documents/{doc_id}")),
        );
        let doc_query = TermQuery::new(facet_term, schema::IndexRecordOption::Basic);
        let doc_query = ConstScoreQuery::new(Box::new(doc_query), 0f32);
        doc_query
            .weight(EnableScoring::Disabled(&self.schema))
            .unwrap();
        let query_terms = query.split_whitespace().collect::<Vec<_>>();

        let mut queries: Vec<_> = query_terms
            .iter()
            .flat_map(|w| {
                let mut vec = Vec::<Box<dyn Query>>::new();
                vec.push(Box::new(FuzzyTermQuery::new(
                    Term::from_field_text(self.fields.body, w),
                    1,
                    true,
                )));
                vec.push(Box::new(
                    RegexQuery::from_pattern(&format!(r#"[^\s]*{w}[^\s]*"#), self.fields.body)
                        .unwrap(),
                ));
                vec
            })
            .map(|q| (Occur::Should, q.box_clone()))
            .collect();

        queries.push((Occur::Must, Box::new(doc_query)));

        let query_parser = QueryParser::for_index(&self.index, vec![self.fields.body]);
        let query = query_parser.parse_query(query).unwrap();

        let searcher = self.reader.searcher();
        let snippet_generator =
            SnippetGenerator::create(&searcher, &query, self.fields.body).unwrap();
        queries.push((Occur::Should, Box::new(query)));

        let q = BooleanQuery::new(queries);

        let mut results = Vec::new();

        for (score, doc_address) in searcher.search(&q, &TopDocs::with_limit(10))? {
            if score == 0f32 {
                continue;
            }
            let doc = searcher.doc(doc_address)?;
            let Some(field) = doc.get_first(self.fields.page) else {
                log::error!("Document {} is incorrectly indexed and page field is missing", doc_id);
                continue;
            };
            let Some(page) = field.as_u64() else {
                log::error!("Document {} has fields of incorrect type. Failed to read page field", doc_id);
                continue;
            };
            let snippet = snippet_generator.snippet_from_doc(&doc).to_html();
            let res = SearchResult {
                page,
                score,
                snippet: match snippet.len() {
                    0 => None,
                    _ => Some(snippet),
                },
            };
            results.push(res);
        }

        Ok(results)
    }
}

#[derive(thiserror::Error)]
pub enum IndexerError {
    #[error(transparent)]
    CreateIndexDirectoryError(#[from] std::io::Error),
    #[error(transparent)]
    OpenIndexError(#[from] tantivy::error::TantivyError),
    #[error("Invalid document index")]
    InvalidDocument,
}

impl std::fmt::Debug for IndexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error::error_chain_fmt(self, f)
    }
}
