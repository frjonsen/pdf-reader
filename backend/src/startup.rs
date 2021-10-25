use std::net::TcpListener;

use crate::configuration::Settings;
use crate::database;
use crate::documents;
use actix_web::{dev::Server, get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Sqlite};

pub struct Application {
    pub port: u16,
    pub server: Server,
}

#[get("health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener, db_pool: Pool<Sqlite>) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    println!("Starting listen on {}", listener.local_addr().unwrap());
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(documents::setup_documents_service())
                    .service(health_check),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = database::get_connection_pool(&configuration);
        database::initialize_database(&connection_pool).await;

        let adress_binding = format!("0.0.0.0:{}", configuration.port);
        let listener = TcpListener::bind(adress_binding)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool)?;

        Ok(Self { server, port })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
