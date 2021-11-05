use std::net::TcpListener;

use crate::configuration::Settings;
use crate::database;
use crate::documents;
use actix_web::{dev::Server, get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
pub struct Application {
    pub port: u16,
    pub server: Server,
}

#[get("health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    configuration: Settings,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let config = web::Data::new(configuration);
    println!("Starting listen on {}", listener.local_addr().unwrap());
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(documents::setup_documents_service())
                    .service(health_check),
            )
            .app_data(db_pool.clone())
            .app_data(config.clone())
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
        let documents_path = configuration.documents_storage_path();
        if !std::path::Path::exists(&documents_path) {
            println!("Path {:?} did not exist. Creating it now", &documents_path);
            tokio::fs::create_dir_all(documents_path).await.unwrap();
        }
    }
}
