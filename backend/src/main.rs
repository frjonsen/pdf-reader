use pdf_reader::configuration::get_configuration;
use pdf_reader::startup::Application;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Launching backend");

    env_logger::init();

    let configuration = get_configuration();

    Application::build(configuration, None)
        .await?
        .run_until_stopped()
        .await
}
