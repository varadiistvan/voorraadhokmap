use axum::{Form, Json};
use axum_macros::debug_handler;
use fake::{Fake, Faker};
use serde::Deserialize;

use crate::admin::ColumnsTemplate;

#[derive(Deserialize, Debug)]
pub struct UpdateCrateOrderRequest {
    section: String,
    crate_ids: Vec<usize>,
}

#[debug_handler]
pub async fn update_crate_order(body: String) -> ColumnsTemplate {
    println!("{body:?}");

    ColumnsTemplate {
        columns: Faker.fake(),
    }
}
