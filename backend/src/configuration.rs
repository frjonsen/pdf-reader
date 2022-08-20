use std::path::PathBuf;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database_location: Option<String>,
    pub database_name: Option<String>,
    pub storage_location: PathBuf,
    pub port: u16,
}

impl Settings {
    pub fn documents_storage_path(&self) -> PathBuf {
        self.storage_location.join("documents")
    }

    pub fn get_database_name(&self) -> String {
        self.database_name
            .clone()
            .unwrap_or_else(|| "pdfreader".to_owned())
    }

    pub fn get_database_location(&self) -> String {
        self.database_location
            .clone()
            .unwrap_or_else(|| "postgres://postgres:password@localhost:5432".to_owned())
    }

    pub fn get_connection_string_with_db(&self) -> String {
        let location = self.get_database_location();
        let database_name = self.get_database_name();
        format!("{}/{}", location, database_name)
    }
}

pub fn get_configuration() -> Settings {
    let mut settings = config::Config::default();
    settings
        .set_default("storage_location", "/pdf_reader")
        .expect("Failed to set default for storage location");

    settings
        .set_default("port", 8080)
        .expect("Failed to set default for port number");

    settings
        .merge(config::Environment::with_prefix("PDF_READER"))
        .expect("Failed to merge in environment");

    settings.try_into().expect("Failed to read configuration")
}
