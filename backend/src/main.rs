use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::Result as AWResult;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

mod documents;
mod models;

use crate::documents::save_document;
use crate::models::DocumentsListItem;

#[get("documents")]
async fn list_documents() -> impl Responder {
    let files = vec![DocumentsListItem {
        id: Uuid::new_v4(),
        name: "adocument.pdf".to_owned(),
    }];
    HttpResponse::Ok().json(files)
}

#[post("documents")]
async fn upload_document(payload: Multipart) -> impl Responder {
    match save_document(payload).await {
        Err(msg) => HttpResponse::InternalServerError().body(msg),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}

#[get("health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("documents/{id}")]
async fn get_document(id: web::Path<String>) -> AWResult<NamedFile> {
    let path = PathBuf::from_str("../deploy/pdf.pdf")?;
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Launching backend");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(list_documents)
                .service(health_check)
                .service(get_document)
                .service(upload_document),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
