use once_cell::sync::Lazy;
use pdf_reader::telemetry::{get_subscriber, init_subscriber};
use std::str::FromStr;

use pdf_reader::configuration::{get_configuration, Settings};
use pdf_reader::database;
use pdf_reader::models::AddBookmarkRequest;
use pdf_reader::startup::Application;
use sqlx::postgres::PgConnectOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tempfile::TempDir;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub config: Settings,
    pub test_id: Uuid,
    pub client: reqwest::Client,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        std::fs::remove_dir_all(self.config.storage_location.clone())
            .expect("Failed to delete temporary storage directory");

        let database_name = self.config.get_database_name();
        let command = format!(r#"DROP DATABASE "{}" WITH (FORCE);"#, database_name);
        let result = std::process::Command::new("psql")
            .env("PGUSER", "postgres")
            .env("PGPASSWORD", "password")
            .env("PGHOST", "localhost")
            .arg("-c")
            .arg(command)
            .output()
            .unwrap();

        if !result.status.success() {
            println!("{:?}", std::str::from_utf8(&result.stderr));
            panic!("Failed to delete database after test");
        }
    }
}

impl TestApp {
    pub async fn post_bookmark(
        &self,
        document_id: Uuid,
        page: i32,
        description: &str,
    ) -> reqwest::Response {
        let body = AddBookmarkRequest {
            description: description.to_owned(),
            page,
        };

        let url = format!(
            "{}/api/documents/{}/bookmarks",
            &self.address,
            document_id.as_hyphenated()
        );
        println!("Posting to {}", url);
        self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .expect("Failed to send bookmark request")
    }

    pub async fn post_document(&self, form_contents: &[u8]) -> reqwest::Response {
        let body = reqwest::multipart::Part::bytes(form_contents.to_owned()).file_name("file.pdf");
        let form = reqwest::multipart::Form::new().part("field1", body);
        let url = format!("{}/api/documents", &self.address);
        self.client
            .post(url)
            .multipart(form)
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub async fn configure_database(config: &mut Settings, test_id: &Uuid) -> PgPool {
    config.database_name = Some(test_id.as_hyphenated().to_string());

    let options = PgConnectOptions::from_str(&config.get_database_location())
        .expect("Failed to parse connection string");

    PgConnection::connect_with(&options)
        .await
        .expect("Failed to connect to Postgres")
        .execute(&*format!(
            r#"CREATE DATABASE "{}""#,
            test_id.as_hyphenated()
        ))
        .await
        .expect("Failed to create database");

    let connection_pool = database::get_connection_pool(config);
    database::initialize_database(&connection_pool).await;

    connection_pool
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let test_id = Uuid::new_v4();
    let mut configuration = get_configuration();
    configuration.port = 0;
    configuration.storage_location = TempDir::new().unwrap().path().to_path_buf();

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
        client: reqwest::Client::new(),
    }
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if let Ok(log_level) = std::env::var("TEST_LOG") {
        let subscriber = get_subscriber(subscriber_name, log_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});
