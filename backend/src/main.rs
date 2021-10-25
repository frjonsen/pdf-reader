use pdf_reader::configuration::get_configuration;
use pdf_reader::startup::Application;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Launching backend");

    env_logger::init();

    let configuration = get_configuration();

    let documents_path = configuration.documents_storage_path();

    if !std::path::Path::exists(&documents_path) {
        println!("Path {:?} did not exist. Creating it now", &documents_path);
        tokio::fs::create_dir_all(documents_path).await.unwrap();
    }

    Application::build(configuration)
        .await?
        .run_until_stopped()
        .await
}
