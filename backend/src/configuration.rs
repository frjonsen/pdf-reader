use std::path::PathBuf;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub storage_location: PathBuf,
}

impl Settings {
    pub fn documents_storage_path(&self) -> PathBuf {
        get_configuration().storage_location.join("documents")
    }
}

pub fn get_configuration() -> Settings {
    let mut settings = config::Config::default();
    settings
        .set_default("storage_location", "/pdf_reader")
        .expect("Failed to set default for storage location");

    settings
        .merge(config::Environment::with_prefix("PDF_READER"))
        .expect("Failed to merge in environment");

    settings.try_into().expect("Failed to read configuration")
}
