use pdf_reader::configuration::{get_configuration, Settings};
use pdf_reader::database;
use pdf_reader::startup::Application;
use sqlx::SqlitePool;

pub struct TestApp {
    pub address: String,
    pub db_pool: SqlitePool,
}

pub async fn configure_database(config: &Settings) -> SqlitePool {
    let connection_pool = database::get_connection_pool(config);
    database::initialize_database(&connection_pool).await;

    connection_pool
}

pub async fn spawn_app() -> TestApp {
    let mut configuration = get_configuration();
    configuration.port = 0;

    let db = configure_database(&configuration).await;

    let app = Application::build(configuration)
        .await
        .expect("Failed to build application");

    let address = format!("http://localhost:{}", app.port);

    let _ = tokio::spawn(app.run_until_stopped());

    TestApp {
        address,
        db_pool: db,
    }
}
