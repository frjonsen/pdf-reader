use actix_web::{web, HttpResponse, ResponseError, Scope};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::error_chain_fmt,
    indexer::{Indexer, IndexerError},
};

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

pub async fn search_document(
    id: web::Path<Uuid>,
    indexer: web::Data<Indexer>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, SearchError> {
    let res = indexer.search_document(&id, &query.q)?;

    Ok(HttpResponse::Ok().json(res))
}

pub fn setup_search_service() -> Scope {
    web::scope("/documents/{document_id}/search").route("", web::get().to(search_document))
}

#[derive(thiserror::Error)]
pub enum SearchError {
    #[error(transparent)]
    SearcherError(#[from] IndexerError),
}

impl std::fmt::Debug for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SearchError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}
