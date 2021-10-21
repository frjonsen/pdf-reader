#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub storage_location: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.set_default("storage_location", "/pdf_reader")?;

    settings.merge(config::Environment::with_prefix("PDF_READER"))?;

    settings.try_into()
}
