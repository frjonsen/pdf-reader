use std::path::PathBuf;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub connection_string: Option<String>,
    pub storage_location: PathBuf,
    pub port: u16,
}

impl Settings {
    pub fn documents_storage_path(&self) -> PathBuf {
        self.storage_location.join("documents")
    }

    pub fn get_connection_string(&self) -> String {
        self.connection_string.clone().unwrap_or_else(|| {
            self.storage_location
                .join("database.db")
                .to_string_lossy()
                .to_string()
        })
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
