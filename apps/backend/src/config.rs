use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
    pub dbname: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RedisConfig {
    pub uri: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthConfig {
    pub jwt_access_secret: Secret<String>,
    pub jwt_access_expires_in: String,
    pub jwt_refresh_secret: Secret<String>,
    pub jwt_refresh_expires_in: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        config.try_deserialize()
    }
}

impl DatabaseConfig {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.dbname)
            .log_statements(tracing::log::LevelFilter::Trace)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
    }

    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect_with(self.with_db())
            .await
    }
}