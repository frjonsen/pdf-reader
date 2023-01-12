use std::path::PathBuf;

use actix_multipart::Multipart;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use anyhow::Context;
use futures::StreamExt;
use futures::TryStreamExt;
use once_cell::sync::Lazy;
use pdfium_render::prelude::Pdfium;
use sqlx::PgPool;
use sqlx::{Postgres, Transaction};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::configuration::Settings;
use crate::error::error_chain_fmt;
use crate::indexer::Indexer;
use crate::indexer::IndexerError;

async fn insert_document<'a>(
    id: Uuid,
    filename: String,
    transaction: &mut Transaction<'a, Postgres>,
) -> Result<(), AddDocumentError> {
    println!("Saving file {} in database", filename);
    sqlx::query!(
        "INSERT INTO Documents (id, name) VALUES ($1, $2)",
        id,
        filename
    )
    .execute(transaction)
    .await
    .context("Failed to insert document")?;

    Ok(())
}
pub async fn upload_document(
    indexer: web::Data<Indexer>,
    pool: web::Data<PgPool>,
    pdfium: web::Data<&Lazy<Pdfium>>,
    config: web::Data<Settings>,
    mut payload: Multipart,
) -> Result<HttpResponse, AddDocumentError> {
    log::info!("Handling incoming documents");
    let mut saved: Vec<Uuid> = Vec::new();
    let mut tx = pool
        .begin()
        .await
        .context("Failed to begin database transaction")?;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let id = uuid::Uuid::new_v4();

        log::debug!("Writing document to disk");
        let res = match save_document_to_disk(&id, &mut field, config.get_ref()).await {
            Err(e) => {
                log::error!("Failed to write document to disk. Unwinding transaction.");
                delete_documents(&saved, config.get_ref());
                return Err(e);
            }
            Ok(f) => f,
        };

        saved.push(id);
        if let Err(e) = insert_document(id, res.0, &mut tx).await {
            log::error!("Failed insert document in database. Unwinding transaction.");
            delete_documents(&saved, config.get_ref());
            return Err(e);
        }

        index_pdf_file(pdfium.as_ref(), &indexer, &res.1, &id).await?;
    }

    let commit_result = tx.commit().await;
    if commit_result.is_err() {
        if !saved.is_empty() {
            delete_documents(&saved, config.get_ref());
        }
        commit_result.context("Failed to commit transaction")?;
    }

    log::debug!("Document successfully added");
    Ok(HttpResponse::Created().finish())
}

async fn save_document_to_disk(
    id: &Uuid,
    field: &mut actix_multipart::Field,
    config: &Settings,
) -> Result<(String, PathBuf), AddDocumentError> {
    let base_path = config.documents_storage_path();
    let file_path = base_path.join(id.to_string()).with_extension("pdf");

    let mut fd = tokio::fs::File::create(&file_path)
        .await
        .context("Failed to create file")?;

    let filename = field
        .content_disposition()
        .get_filename()
        .ok_or(AddDocumentError::MissingFilename)?
        .to_owned();

    println!("Writing file {} to disk", filename);

    while let Some(chunk) = field.next().await {
        let chunk = chunk.context("Failed to read chunk")?;
        fd.write_all(&chunk)
            .await
            .context("Failed to write chunk to storage")?;
    }

    Ok((filename, file_path))
}

fn delete_documents(document_ids: &[Uuid], config: &Settings) {
    if document_ids.is_empty() {
        return;
    }

    let documents_path = config.documents_storage_path();
    let documents_on_disk = match std::fs::read_dir(documents_path) {
        Err(e) => {
            println!("Failed to read documents directory {}", e);
            return;
        }
        Ok(r) => r,
    };

    let ids = document_ids.iter().map(Uuid::to_string).collect::<Vec<_>>();

    let to_delete = documents_on_disk.filter_map(Result::ok).filter(|f| {
        let file_name = f.file_name();
        let lossy = file_name.to_string_lossy();
        ids.iter().any(|i| lossy.starts_with(i))
    });

    let failed_deletes = to_delete
        .map(|f| std::fs::remove_file(f.path()))
        .filter_map(Result::err)
        .collect::<Vec<_>>();

    for failed in failed_deletes {
        println!("Failed to delete: {}", failed);
    }
}
pub async fn index_pdf_file(
    pdfium: &Pdfium,
    indexer: &Indexer,
    file: &PathBuf,
    doc_id: &Uuid,
) -> Result<(), AddDocumentError> {
    log::info!("Indexing new document {}", doc_id);
    let pdf = pdfium
        .load_pdf_from_file(file, None)
        .context("Failed to load pdf file")?;

    let mut writer = indexer.get_writer().await?;

    for (page_nr, p) in pdf.pages().iter().enumerate() {
        let text = p.text().context("Failed to read pdf file")?.all();
        writer.index_page(doc_id, page_nr as u64 + 1, &text)?;
    }

    writer.commit()?;
    log::info!("Index of document {} committed", doc_id);

    Ok(())
}

#[derive(thiserror::Error)]
pub enum AddDocumentError {
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
    #[error("Failed to index document contents")]
    IndexingError(#[from] IndexerError),
    #[error("Uploaded file must have a name")]
    MissingFilename,
}

impl std::fmt::Debug for AddDocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for AddDocumentError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::MissingFilename => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
