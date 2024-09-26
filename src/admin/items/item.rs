use std::{error::Error, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, State},
    http::Response,
    Form,
};
use serde::Deserialize;
use sqlx::{query, PgPool};

use crate::admin::{
    get_columns, get_item_by_name, get_items, query_error_to_internal, AdminTemplate, ItemTemplate,
    ItemsTemplate,
};

#[derive(Debug, Deserialize)]
pub struct ChangeNameForm {
    name: String,
}

pub async fn change_name(
    State(pool): State<Arc<PgPool>>,
    Path(old_name): Path<String>,
    Form(body): Form<ChangeNameForm>,
) -> Result<ItemTemplate, Response<Body>> {
    let item = get_item_by_name(&old_name, &pool).await?;

    let _succ = query!(
        "UPDATE items SET name = $1 WHERE name = $2",
        body.name,
        old_name
    )
    .execute(&*pool)
    .await
    .map_err(query_error_to_internal)?;

    Ok(ItemTemplate { item })
}

pub async fn delete_item(
    State(pool): State<Arc<PgPool>>,
    Path(item_name): Path<String>,
) -> Result<ItemsTemplate, Response<Body>> {
    let _item = get_item_by_name(&item_name, &pool).await?;

    let _succ = query!("DELETE FROM items WHERE items.name = $1", item_name)
        .execute(&*pool)
        .await
        .map_err(query_error_to_internal)?;

    Ok(ItemsTemplate {
        items: get_items(&pool).await?,
    })
}

#[derive(Debug, Deserialize)]
pub struct CreateItemForm {
    name: String,
}
pub async fn create_item(
    State(pool): State<Arc<PgPool>>,
    Form(body): Form<CreateItemForm>,
) -> Result<ItemsTemplate, Response<Body>> {
    let _succ = query!(
        "INSERT INTO items (name, aliases) VALUES ($1, '{}')",
        body.name
    )
    .execute(&*pool)
    .await
    .map_err(query_error_to_internal)?;

    Ok(ItemsTemplate {
        items: get_items(&pool).await?,
    })
}
