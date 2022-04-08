use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: Secret<String>,
    pub port: String,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<AppConfig, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration.yml"))?;
    settings.try_into()
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}
