use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize, Debug)]
struct Config {
    admin_password: String,
    database_url: String,
}

#[derive(Debug)]
struct AppState {
    admin_password: String,
    pool: sqlx::SqlitePool,
}

mod admin;
mod models;
mod public;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let config = envy::from_env::<Config>().unwrap();

    let admin_routes = Router::new()
        .route("/", get(admin::index))
        .route("/item", post(admin::create_item))
        .route(
            "/item/:name",
            put(admin::change_name)
                .post(admin::create_alias)
                .delete(admin::delete_item),
        )
        .route("/item/:item_name/:alias", delete(admin::delete_alias))
        .route(
            "/crates/update-crate-order",
            post(admin::update_crate_order),
        );

    let public_routes = Router::new()
        .route("/", get(public::index))
        .route("/rectangles/:term", get(public::get_crates));

    let app = Router::new()
        .nest("/", public_routes)
        .nest("/admin", admin_routes)
        .with_state(Arc::new(AppState {
            admin_password: config.admin_password,
            pool: SqlitePool::connect(&config.database_url).await.unwrap(),
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
