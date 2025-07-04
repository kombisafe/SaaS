
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::AppError;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

pub fn create_token(
    user_id: Uuid,
    secret: &str,
    expires_in: &str,
) -> Result<String, AppError> {
    let now = Utc::now();
    let expiration = Duration::from_std(duration_str::parse(expires_in)?).map_err(|_| AppError::InternalServerError)?;
    let claims = Claims {
        sub: user_id,
        exp: (now + expiration).timestamp(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
    };
    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(AppError::Jwt)
}

pub fn validate_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}
