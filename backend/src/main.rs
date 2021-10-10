use actix_files::NamedFile;
use actix_web::Result as AWResult;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::path::PathBuf;
use std::str::FromStr;

#[get("documents")]
async fn list_documents() -> impl Responder {
    let files = vec!["adocument.pdf"];
    HttpResponse::Ok().json(files)
}

#[get("health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("documents/{id}")]
async fn get_document(id: web::Path<String>) -> AWResult<NamedFile> {
    let path = PathBuf::from_str("../deploy/dummy.pdf")?;
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
                .service(get_document),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
