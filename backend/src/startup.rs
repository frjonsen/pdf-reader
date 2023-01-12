use std::net::TcpListener;

use crate::configuration::Settings;
use crate::database;
use crate::indexer::Indexer;
use crate::routes::{bookmarks, documents, search};
use actix_web::middleware::Logger;
use actix_web::{dev::Server, get, web, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::Lazy;
use pdfium_render::prelude::Pdfium;
use sqlx::PgPool;

pub struct Application {
    pub port: u16,
    pub server: Server,
}

#[get("health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

static PDFIUM: Lazy<Pdfium> = Lazy::new(|| {
    log::info!("Binding pdfium");
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .unwrap(),
    );
    log::info!("Pdfium successfully bound");
    pdfium
});

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    configuration: Settings,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let indexer = web::Data::new(
        Indexer::new(configuration.documents_contents_path()).expect("Failed to set up indexer"),
    );
    let config = web::Data::new(configuration);
    let pdfium = web::Data::new(&PDFIUM);
    log::info!("Starting listen on {}", listener.local_addr().unwrap());
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(search::setup_search_service())
                    .service(bookmarks::setup_bookmarks_service())
                    .service(documents::setup_documents_service())
                    .service(health_check),
            )
            .app_data(db_pool.clone())
            .app_data(config.clone())
            .app_data(pdfium.clone())
            .app_data(indexer.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
impl Application {
    pub async fn build(
        configuration: Settings,
        db_pool: Option<PgPool>,
    ) -> Result<Self, std::io::Error> {
        Application::ensure_storage_path(&configuration).await;
        let connection_pool =
            db_pool.unwrap_or_else(|| database::get_connection_pool(&configuration));
        database::initialize_database(&connection_pool).await;

        let adress_binding = format!("0.0.0.0:{}", configuration.port);
        let listener = TcpListener::bind(adress_binding)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, configuration)?;

        Ok(Self { server, port })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub async fn ensure_storage_path(configuration: &Settings) {
        let required_paths = [
            configuration.documents_storage_path(),
            configuration.documents_contents_path(),
        ];
        for path in required_paths.into_iter() {
            if !std::path::Path::exists(&path) {
                println!("Path {:?} did not exist. Creating it now", &path);
                tokio::fs::create_dir_all(path).await.unwrap();
            }
        }
    }
}
