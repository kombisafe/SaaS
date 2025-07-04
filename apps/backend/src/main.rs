use backend::{
    config::AppConfig,
    db,
    redis,
    routes::create_router,
    state::AppState,
};
use std::net::SocketAddr;
use tokio;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialiser le logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Charger la configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env()?;

    // Créer le pool de connexions à la base de données
    let pool = db::create_pool(&config.database).await?;
    tracing::info!("Database pool created successfully.");

    // Créer le client Redis
    let redis_client = redis::create_client(&config.redis)?;
    tracing::info!("Redis client created successfully.");

    // Créer l'état de l'application
    let state = AppState {
        pool,
        config: config.clone(),
        redis: redis_client,
    };

    // Définir les routes de notre application
    let app = create_router(state);

    // Définir l'adresse et le port
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>().map_err(|e| format!("Invalid IP address in config: {}", e))?,
        config.server.port,
    ));
    tracing::info!("Server listening on {}", addr);

    // Lancer le serveur
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}