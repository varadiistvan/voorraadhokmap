use std::{error::Error, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode},
    Form,
};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
use serde::Deserialize;
use sqlx::query;

use crate::{
    admin::{get_item_by_name, query_error_to_internal, ItemTemplate},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateAliasForm {
    alias: String,
}

pub async fn create_alias(
    State(state): State<Arc<AppState>>,
    Path(item_name): Path<String>,
    Form(body): Form<CreateAliasForm>,
) -> Result<ItemTemplate, Response<Body>> {
    let mut item = get_item_by_name(&item_name, &state).await?;

    let _succ = query!(
        "INSERT INTO item_aliases (item_name, alias) VALUES (?, ?)",
        item_name,
        body.alias
    )
    .execute(&state.pool)
    .await
    .map_err(query_error_to_internal)?;

    item.aliases.push(body.alias);

    Ok(ItemTemplate { item })
}

pub async fn delete_alias(
    auth_header: Result<TypedHeader<Authorization<Basic>>, impl Error>,
    State(state): State<Arc<AppState>>,
    Path((item_name, alias)): Path<(String, String)>,
) -> Result<ItemTemplate, Response<Body>> {
    let mut item = get_item_by_name(&item_name, &state).await?;

    if !item.aliases.contains(&alias) {
        return Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap());
    }

    let _succ = query!(
        "DELETE FROM item_aliases WHERE item_aliases.item_name = ? AND item_aliases.alias = ?",
        item_name,
        alias
    )
    .execute(&state.pool)
    .await
    .map_err(query_error_to_internal)?;

    item.aliases.retain(|e| e != &alias);

    Ok(ItemTemplate { item })
}
