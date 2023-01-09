mod get_all;
mod get_by_id;
mod patch;
mod post;

use actix_web::{web, Scope};

use self::{
    get_all::list_documents, get_by_id::get_document, patch::update_document_status,
    post::upload_document,
};

pub fn setup_documents_service() -> Scope {
    web::scope("/documents")
        .route("{id}", web::get().to(get_document))
        .route("{id}", web::patch().to(update_document_status))
        .route("", web::get().to(list_documents))
        .route("", web::post().to(upload_document))
}
