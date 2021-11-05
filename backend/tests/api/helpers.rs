use std::path::PathBuf;
use std::str::FromStr;

use pdf_reader::configuration::{get_configuration, Settings};
use pdf_reader::database;
use pdf_reader::startup::Application;
use sqlx::postgres::PgConnectOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub config: Settings,
    pub test_id: Uuid,
}

impl TestApp {
    pub async fn drop(&mut self) {
        self.db_pool.close().await;
        let options = PgConnectOptions::from_str(&self.config.get_database_location())
            .expect("Failed to parse connection string");

        PgConnection::connect_with(&options)
            .await
            .expect("Failed to connect to Postgres")
            .execute(&*format!(
                r#"DROP DATABASE "{}""#,
                self.test_id.to_hyphenated().to_string()
            ))
            .await
            .expect("Failed to drop database on cleanup");

        std::fs::remove_dir_all(self.config.storage_location.clone())
            .expect("Failed to delete temporary storage directory");
    }
}

pub async fn configure_database(config: &mut Settings, test_id: &Uuid) -> PgPool {
    config.database_name = Some(test_id.to_hyphenated().to_string());

    let options = PgConnectOptions::from_str(&config.get_database_location())
        .expect("Failed to parse connection string");

    PgConnection::connect_with(&options)
        .await
        .expect("Failed to connect to Postgres")
        .execute(&*format!(
            r#"CREATE DATABASE "{}""#,
            test_id.to_hyphenated().to_string()
        ))
        .await
        .expect("Failed to create database");

    let connection_pool = database::get_connection_pool(config);
    database::initialize_database(&connection_pool).await;

    connection_pool
}

pub fn setup_temp_storage(test_id: &Uuid) -> PathBuf {
    let path = std::env::temp_dir().join(test_id.to_hyphenated().to_string());
    std::fs::create_dir(&path).unwrap();
    path
}

pub async fn spawn_app() -> TestApp {
    let test_id = Uuid::new_v4();
    let mut configuration = get_configuration();
    configuration.port = 0;
    configuration.storage_location = setup_temp_storage(&test_id);

    let db = configure_database(&mut configuration, &test_id).await;

    let app = Application::build(configuration.clone(), Some(db.clone()))
        .await
        .expect("Failed to build application");

    let address = format!("http://localhost:{}", app.port);

    let _ = tokio::spawn(app.run_until_stopped());

    TestApp {
        address,
        db_pool: db,
        config: configuration,
        test_id,
    }
}
