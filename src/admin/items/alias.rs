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
use sqlx::{query, PgPool};

use crate::admin::{get_item_by_name, query_error_to_internal, ItemTemplate};

#[derive(Deserialize)]
pub struct CreateAliasForm {
    alias: String,
}

pub async fn create_alias(
    State(pool): State<Arc<PgPool>>,
    Path(item_name): Path<String>,
    Form(body): Form<CreateAliasForm>,
) -> Result<ItemTemplate, Response<Body>> {
    let mut item = get_item_by_name(&item_name, &pool).await?;

    let _succ = query!(
        "UPDATE items SET aliases = aliases || $1::text WHERE name = $2",
        body.alias,
        item_name,
    )
    .execute(&*pool)
    .await
    .map_err(query_error_to_internal)?;

    item.aliases.push(body.alias);

    Ok(ItemTemplate { item })
}

pub async fn delete_alias(
    auth_header: Result<TypedHeader<Authorization<Basic>>, impl Error>,
    State(pool): State<Arc<PgPool>>,
    Path((item_name, alias)): Path<(String, String)>,
) -> Result<ItemTemplate, Response<Body>> {
    let mut item = get_item_by_name(&item_name, &pool).await?;

    if !item.aliases.contains(&alias) {
        return Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap());
    }

    let _succ = query!(
        "UPDATE items set aliases = array_remove(aliases, $1) WHERE items.name = $2",
        alias,
        item_name
    )
    .execute(&*pool)
    .await
    .map_err(query_error_to_internal)?;

    item.aliases.retain(|e| e != &alias);

    Ok(ItemTemplate { item })
}
