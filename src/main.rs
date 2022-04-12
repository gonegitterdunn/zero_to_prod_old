use zero_to_prod::app_config::get_configuration;
use zero_to_prod::startup::build;
use zero_to_prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero_to_prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Configuration not loaded");
    let server = build(configuration).await?;
    server.await?;
    Ok(())
}
