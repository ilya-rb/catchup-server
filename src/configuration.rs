use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgSslMode;
use sqlx::ConnectOptions;

use crate::environment::Environment;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub http_client: HttpClientSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct AppSettings {
    pub name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct HttpClientSettings {
    pub timeout_millis: u64,
}

pub fn read_configuration() -> Result<Settings, config::ConfigError> {
    use config::Config;
    use config::File;

    let base_path = std::env::current_dir().expect("Failed to find current dir");
    let config_dir = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| String::from(Environment::Dev.as_str()))
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_config_file = format!("{}.yaml", environment.as_str());
    let settings = Config::builder()
        .add_source(File::from(config_dir.join("base.yaml")))
        .add_source(File::from(config_dir.join(environment_config_file)))
        .add_source(
            // Variable format APP_APP_SETTINGS__PORT=5001 -> Settings.app_settings.port
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.database_name)
            .log_statements(tracing_log::log::LevelFilter::Trace)
    }
}

impl HttpClientSettings {
    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_millis)
    }
}
