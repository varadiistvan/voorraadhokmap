use std::sync::Arc;

use axum::Router;
use middleware::AuthLayer;
use serde::Deserialize;
use sqlx::SqlitePool;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Deserialize, Debug)]
struct Config {
    admin_password: String,
    admin_username: String,
    database_url: String,
}

#[derive(Debug)]
struct AppState {
    pool: sqlx::SqlitePool,
}

mod admin;
mod middleware;
mod models;
mod public;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let config = envy::from_env::<Config>().unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = Router::new()
        .nest("/", public::get_public_routes())
        .nest(
            "/admin",
            admin::get_admin_routes(AuthLayer::new(config.admin_username, config.admin_password)),
        )
        .with_state(Arc::new(AppState {
            pool: SqlitePool::connect(&config.database_url).await.unwrap(),
        }))
        .layer(ServiceBuilder::new().layer(CompressionLayer::new()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
