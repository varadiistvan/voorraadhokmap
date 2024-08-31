use std::{error::Error, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, State},
    http::Response,
    Form,
};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
use serde::Deserialize;
use sqlx::query;

use crate::{
    admin::{
        authenticate, get_item_by_name, get_items, query_error_to_internal, AdminTemplate,
        ColumnsTemplate, ItemTemplate, ItemsTemplate,
    },
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct ChangeNameForm {
    name: String,
}

pub async fn change_name(
    auth_header: Result<TypedHeader<Authorization<Basic>>, impl Error>,
    State(state): State<Arc<AppState>>,
    Path(old_name): Path<String>,
    Form(body): Form<ChangeNameForm>,
) -> Result<ItemTemplate, Response<Body>> {
    authenticate(auth_header, &state.admin_password)?;

    let item = get_item_by_name(&old_name, &state).await?;

    let _succ = query!(
        "UPDATE items SET name = ? WHERE name = ?",
        body.name,
        old_name
    )
    .execute(&state.pool)
    .await
    .map_err(query_error_to_internal)?;

    Ok(ItemTemplate { item })
}

pub async fn delete_item(
    auth_header: Result<TypedHeader<Authorization<Basic>>, impl Error>,
    State(state): State<Arc<AppState>>,
    Path(item_name): Path<String>,
) -> Result<ItemsTemplate, Response<Body>> {
    authenticate(auth_header, &state.admin_password)?;

    let _item = get_item_by_name(&item_name, &state).await?;

    let _succ = query!("DELETE FROM items WHERE items.name = ?", item_name)
        .execute(&state.pool)
        .await
        .map_err(query_error_to_internal)?;

    Ok(ItemsTemplate {
        items: get_items(&state).await?,
    })
}

#[derive(Debug, Deserialize)]
pub struct CreateItemForm {
    name: String,
}
pub async fn create_item(
    auth_header: Result<TypedHeader<Authorization<Basic>>, impl Error>,
    State(state): State<Arc<AppState>>,
    Form(body): Form<CreateItemForm>,
) -> Result<ItemsTemplate, Response<Body>> {
    authenticate(auth_header, &state.admin_password)?;

    let _succ = query!("INSERT INTO items (name) VALUES (?)", body.name)
        .execute(&state.pool)
        .await
        .map_err(query_error_to_internal)?;

    Ok(ItemsTemplate {
        items: get_items(&state).await?,
    })
}
