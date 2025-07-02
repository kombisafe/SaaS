# SaaS Project

Ce projet est un monorepo pour une application SaaS complète.

## Structure

-   `/apps/backend`: API en Rust (Axum)
-   `/apps/frontend`: Application web en Next.js
-   `/packages/*`: Paquets partagés (types, config, etc.)
-   `/infrastructure`: Infrastructure (Docker, Terraform)
-   `/docs`: Documentation

## Démarrage rapide

1.  **Installer les dépendances :**
    ```bash
    pnpm install
    ```

2.  **Lancer l'environnement de développement :**
    ```bash
    docker-compose up -d
    pnpm dev
    ```
