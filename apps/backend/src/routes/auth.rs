use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite, CookieJar};
use secrecy::{ExposeSecret, Secret};
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use crate::utils::token::create_jwt_token;

use crate::{
    auth::{password},
    errors::AppError,
    models::user::{CreateUser, User},
    state::AppState,
};

#[derive(serde::Deserialize)]
pub struct AuthPayload {
    email: String,
    password: Secret<String>,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Response, AppError> {
    let hashed_password = password::hash_password(Secret::new(payload.password))
        .await
        .map_err(AppError::Password)?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *",
        payload.email,
        hashed_password
    )
    .fetch_one(&state.pool)
    .await?;

    let access_token = create_jwt_token(
        user.id,
        state.config.auth.jwt_access_secret.expose_secret(),
        &state.config.auth.jwt_access_expires_in,
    )?;

    let refresh_token = create_jwt_token(
        user.id,
        state.config.auth.jwt_refresh_secret.expose_secret(),
        &state.config.auth.jwt_refresh_expires_in,
    )?;

    let mut redis_conn: MultiplexedConnection = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(AppError::Redis)?;

    redis_conn
        .set_ex::<&str, String, ()>(
            &refresh_token,
            user.id.to_string(),
            60 * 60 * 24 * 7,
        )
        .await
        .map_err(AppError::Redis)?;

    let access_cookie = Cookie::build(("access_token", access_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    let mut response = (StatusCode::CREATED, Json(serde_json::json!({"user": user}))).into_response();
    response
        .headers_mut()
        .append("set-cookie", access_cookie.to_string().parse().unwrap());
    response
        .headers_mut()
        .append("set-cookie", refresh_cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Response, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let is_valid = password::verify_password(payload.password, &user.password_hash)
        .await
        .map_err(AppError::Password)?;

    if !is_valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let access_token = create_jwt_token(
        user.id,
        state.config.auth.jwt_access_secret.expose_secret(),
        &state.config.auth.jwt_access_expires_in,
    )?;

    let refresh_token = create_jwt_token(
        user.id,
        state.config.auth.jwt_refresh_secret.expose_secret(),
        &state.config.auth.jwt_refresh_expires_in,
    )?;

    let mut redis_conn: MultiplexedConnection = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(AppError::Redis)?;

    redis_conn
        .set_ex::<&str, String, ()>(
            &refresh_token,
            user.id.to_string(),
            60 * 60 * 24 * 7,
        )
        .await
        .map_err(AppError::Redis)?;

    let access_cookie = Cookie::build(("access_token", access_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    let mut response = (StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response();
    response
        .headers_mut()
        .append("set-cookie", access_cookie.to_string().parse().unwrap());
    response
        .headers_mut()
        .append("set-cookie", refresh_cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn refresh(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Response, AppError> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or_else(|| AppError::Unauthorized("Missing refresh token".to_string()))?;

    let mut redis_conn: MultiplexedConnection = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(AppError::Redis)?;

    let user_id: String = redis_conn
        .get::<_, String>(&refresh_token)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid refresh token".to_string()))?;

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id.parse::<uuid::Uuid>().unwrap()
    )
    .fetch_one(&state.pool)
    .await?;

    let access_token = create_jwt_token(
        user.id,
        state.config.auth.jwt_access_secret.expose_secret(),
        &state.config.auth.jwt_access_expires_in,
    )?;

    let access_cookie = Cookie::build(("access_token", access_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    let mut response = (StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response();
    response
        .headers_mut()
        .append("set-cookie", access_cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Response, AppError> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or_else(|| AppError::Unauthorized("Missing refresh token".to_string()))?;

    let mut redis_conn: MultiplexedConnection = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(AppError::Redis)?;

    redis_conn
        .del::<&str, ()>(&refresh_token)
        .await
        .map_err(AppError::Redis)?;

    let access_cookie = Cookie::build(("access_token", ""))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    let mut response = (StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response();
    response
        .headers_mut()
        .append("set-cookie", access_cookie.to_string().parse().unwrap());
    response
        .headers_mut()
        .append("set-cookie", refresh_cookie.to_string().parse().unwrap());

    Ok(response)
}
