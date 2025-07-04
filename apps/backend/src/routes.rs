pub mod auth;

use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

use self::auth::{login, logout, refresh, register};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .route("/auth/logout", post(logout))
        .with_state(state)
}

async fn root() -> &'static str {
    "Welcome to the SaaS API"
}