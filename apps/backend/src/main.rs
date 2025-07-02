use tokio;

#[tokio::main]
async fn main() {
    // Charger les variables d'environnement
    dotenvy::dotenv().expect("Failed to read .env file");

    println!("Starting server...");

    // TODO: Initialiser le pool de connexion à la base de données
    // TODO: Définir les routes Axum
    // TODO: Lancer le serveur
}
