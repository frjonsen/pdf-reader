use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("documents")]
async fn list_documents() -> impl Responder {
    let files = vec!["adocument.pdf"];
    HttpResponse::Ok().json(files)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Launching backend");
    HttpServer::new(|| App::new().service(web::scope("/api").service(list_documents)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
