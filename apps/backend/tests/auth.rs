use reqwest::{Client, StatusCode};
use reqwest_cookie_store::CookieStoreMutex;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn register_login_refresh_logout() {
    // Crée un store de cookies thread-safe
    let cookie_store = CookieStoreMutex::default();

    // Crée le client reqwest avec gestion des cookies
    let client = Client::builder()
        .cookie_provider(Arc::new(cookie_store))
        .build()
        .unwrap();

    // 🔐 REGISTER
    let res = client
        .post("http://localhost:8000/auth/register")
        .json(&json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    // 🔐 LOGIN
    let res = client
        .post("http://localhost:8000/auth/login")
        .json(&json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 🔄 REFRESH
    let res = client
        .post("http://localhost:8000/auth/refresh")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 🚪 LOGOUT
    let res = client
        .post("http://localhost:8000/auth/logout")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
