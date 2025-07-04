
use crate::config::AppConfig;
use axum::extract::FromRef;
use redis::Client;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: AppConfig,
    pub redis: Client,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for AppConfig {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

impl FromRef<AppState> for Client {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}
