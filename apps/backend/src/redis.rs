
use crate::{config::RedisConfig, errors::AppError};
use redis::Client;

pub fn create_client(config: &RedisConfig) -> Result<Client, AppError> {
    redis::Client::open(config.uri.as_ref()).map_err(AppError::Redis)
}
