use std::path::PathBuf;

use pdf_reader::configuration::{get_configuration, Settings};
use pdf_reader::database;
use pdf_reader::startup::Application;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: SqlitePool,
    pub config: Settings,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        std::fs::remove_dir_all(self.config.storage_location.clone())
            .expect("Failed to delete temporary storage directory");
    }
}

pub async fn configure_database(config: &Settings) -> SqlitePool {
    let connection_pool = database::get_connection_pool(config);
    database::initialize_database(&connection_pool).await;

    connection_pool
}

pub fn setup_temp_storage() -> PathBuf {
    let dir_name = Uuid::new_v4().to_hyphenated().to_string();
    let path = std::env::temp_dir().join(dir_name);
    std::fs::create_dir(&path).unwrap();
    path
}

pub async fn spawn_app() -> TestApp {
    let mut configuration = get_configuration();
    configuration.port = 0;
    configuration.storage_location = setup_temp_storage();

    let db = configure_database(&configuration).await;

    let app = Application::build(configuration.clone(), Some(db.clone()))
        .await
        .expect("Failed to build application");

    let address = format!("http://localhost:{}", app.port);

    let _ = tokio::spawn(app.run_until_stopped());

    TestApp {
        address,
        db_pool: db,
        config: configuration,
    }
}
