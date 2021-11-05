use crate::configuration::Settings;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgPool};
use std::str::FromStr;

pub fn get_connection_pool(settings: &Settings) -> PgPool {
    let connection_string = settings.get_connection_string_with_db();
    let mut options =
        PgConnectOptions::from_str(&connection_string).expect("Failed to parse database url");
    options.log_statements(log::LevelFilter::Debug);

    PgPool::connect_lazy_with(options)
}

pub async fn initialize_database(pool: &PgPool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Failed to run migration");
}
