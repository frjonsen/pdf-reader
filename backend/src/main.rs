use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx;

mod configuration;
mod database;
mod documents;
mod models;

#[get("health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Launching backend");

    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let db_pool = database::get_connection_pool(&configuration);
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Failed to run migration");
    let db_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(documents::setup_documents_service())
                    .service(health_check),
            )
            .app_data(db_pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
