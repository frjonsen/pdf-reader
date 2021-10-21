use crate::configuration::Settings;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{path::PathBuf, str::FromStr};

pub fn get_connection_pool(settings: &Settings) -> SqlitePool {
    let base_path = PathBuf::from_str(&settings.storage_location).expect("Base path was not valid");
    println!("{:?}", base_path);
    let database_path_buf = base_path.join("database.db");

    let options = SqliteConnectOptions::from_str(&database_path_buf.to_string_lossy())
        .expect("Failed to parse database url")
        .create_if_missing(true);

    SqlitePool::connect_lazy_with(options)
}
