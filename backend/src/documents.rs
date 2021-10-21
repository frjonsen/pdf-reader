use crate::models::DocumentsListItem;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::Result as AWResult;
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use futures::TryStreamExt;
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

fn get_storage_path() -> String {
    std::env::var("DOCUMENT_STORAGE_PATH").unwrap_or("/documents".to_owned())
}

pub async fn save_document(
    pool: web::Data<SqlitePool>,
    mut payload: Multipart,
) -> Result<(), String> {
    let base_path = get_storage_path();

    while let Ok(Some(field)) = payload.try_next().await {
        if let Some(content_disposition) = field.content_disposition() {
            if let Some(filename) = content_disposition.get_filename() {
                println!("Got file {}", filename);
            }
        }
    }

    Ok(())
}
#[get("")]
async fn list_documents() -> impl Responder {
    let files = vec![DocumentsListItem {
        id: Uuid::new_v4(),
        name: "adocument.pdf".to_owned(),
    }];
    HttpResponse::Ok().json(files)
}

#[post("")]
async fn upload_document(pool: web::Data<SqlitePool>, payload: Multipart) -> impl Responder {
    match save_document(pool, payload).await {
        Err(msg) => HttpResponse::InternalServerError().body(msg),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}

#[get("{id}")]
async fn get_document(id: web::Path<String>) -> AWResult<NamedFile> {
    let path = PathBuf::from_str("../deploy/pdf.pdf")?;
    Ok(NamedFile::open(path)?)
}

pub fn setup_documents_service() -> Scope {
    web::scope("/documents")
        .service(get_document)
        .service(list_documents)
        .service(upload_document)
}
