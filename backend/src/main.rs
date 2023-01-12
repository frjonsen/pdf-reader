use pdf_reader::configuration::get_configuration;
use pdf_reader::startup::Application;
use pdf_reader::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("pdf_reader".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);
    println!("Launching backend");

    log::info!("Logging initialized");

    let configuration = get_configuration();

    Application::build(configuration, None)
        .await?
        .run_until_stopped()
        .await
}
