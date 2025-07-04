use crate::config::DatabaseConfig;
use crate::errors::AppError;
use sqlx::PgPool;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, AppError> {
    config.create_pool().await.map_err(AppError::Sqlx)
}