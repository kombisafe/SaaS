use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use secrecy::{ExposeSecret, Secret};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Argon2 error")]
    Argon2Error(#[from] argon2::password_hash::Error),
}

pub async fn hash_password(password: Secret<String>) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .map_err(PasswordError::Argon2Error)?
        .to_string();
    Ok(password_hash)
}

pub async fn verify_password(
    password: Secret<String>,
    hash: &str,
) -> Result<bool, PasswordError> {
    let parsed_hash = PasswordHash::new(hash).map_err(PasswordError::Argon2Error)?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.expose_secret().as_bytes(), &parsed_hash)
        .map_err(PasswordError::Argon2Error)
        .is_ok())
}
