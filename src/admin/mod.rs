mod items;
use std::{collections::HashMap, error::Error, sync::Arc};

use axum::{
    body::Body,
    extract::State,
    http::Response,
    routing::{delete, get, post, put},
    Router,
};
use fake::{faker, Fake, Faker};
pub use items::*;

mod columns;
pub use columns::*;

mod templates;
use sqlx::{pool, PgPool};
use templates::*;

mod utils;
use tower_http::trace::TraceLayer;
use utils::*;

use crate::{
    middleware::AuthLayer,
    models::{Column, Crate, CrateType, Rotation, Section},
};

pub async fn index(State(pool): State<Arc<PgPool>>) -> Result<AdminTemplate, Response<Body>> {
    let items = get_items(&pool).await?;

    let columns = HashMap::from([
        (
            Section::A,
            vec![Faker.fake(), Faker.fake(), Faker.fake(), Faker.fake()],
        ),
        (Section::B, vec![Faker.fake(), Faker.fake()]),
        (Section::C, vec![]),
        (Section::D, vec![]),
        (Section::E, vec![]),
    ]);

    Ok(AdminTemplate { items, columns })
}

pub fn get_admin_routes(auth: AuthLayer) -> Router<Arc<PgPool>> {
    Router::new()
        .route("/", get(index))
        .route("/item", post(create_item))
        .route(
            "/item/:name",
            put(change_name).post(create_alias).delete(delete_item),
        )
        .route("/item/:item_name/:alias", delete(delete_alias))
        .route("/crates/update-crate-order", post(update_crate_order))
        .layer(auth)
}
