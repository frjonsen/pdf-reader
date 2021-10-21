use crate::configuration::Settings;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

pub fn get_connection_pool(settings: &Settings) -> SqlitePool {
    let database_path_buf = settings.storage_location.join("database.db");

    let options = SqliteConnectOptions::from_str(&database_path_buf.to_string_lossy())
        .expect("Failed to parse database url")
        .create_if_missing(true);

    SqlitePool::connect_lazy_with(options)
}
