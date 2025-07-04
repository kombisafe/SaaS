// saas-project/apps/backend/src/utils/token.rs

use crate::auth::jwt;
use crate::errors::AppError;

pub fn create_jwt_token(
    user_id: uuid::Uuid,
    secret: &str,
    expires_in: &str,
) -> Result<String, AppError> {
    jwt::create_token(user_id, secret, expires_in).map_err(|e| {
        // adapte cette ligne selon la nature de l'erreur retournée par create_token
        AppError::Unauthorized(format!("Token creation failed: {}", e))
    })
}
