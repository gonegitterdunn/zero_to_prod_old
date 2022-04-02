use sqlx::PgPool;
use std::net::TcpListener;
use zero_to_prod::app_config::get_configuration;
use zero_to_prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Configuration not loaded");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Could not connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
