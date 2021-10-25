use crate::configuration::Settings;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
use std::str::FromStr;

pub fn get_connection_pool(settings: &Settings) -> SqlitePool {
    let connection_string = settings.get_connection_string();

    let mut options = SqliteConnectOptions::from_str(&connection_string)
        .expect("Failed to parse database url")
        .create_if_missing(true);
    options.log_statements(log::LevelFilter::Debug);

    SqlitePool::connect_lazy_with(options)
}

pub async fn initialize_database(pool: &SqlitePool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Failed to run migration");
}
