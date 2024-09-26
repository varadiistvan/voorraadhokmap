use std::collections::HashMap;

use axum::{
    body::Body,
    http::{Response, StatusCode},
};
use sqlx::{query, query_as, PgPool};

use crate::{
    models::{Column, Crate, CrateType, Item, Rotation, Section},
    public::get_crates,
};

pub fn query_error_to_internal(e: sqlx::Error) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("{e:?}")))
        .unwrap()
}

pub async fn get_items(pool: &PgPool) -> Result<Vec<Item>, Response<Body>> {
    let items = query_as!(
        Item,
        r#"
            SELECT
                items.name,
                items.aliases
            FROM items
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(query_error_to_internal)?;

    Ok(items)
}

pub async fn get_item_by_name(item_name: &str, pool: &PgPool) -> Result<Item, Response<Body>> {
    let Ok(item) = query_as!(
        Item,
        r#"
            SELECT 
                items.name,
                items.aliases
            FROM items
            WHERE items.name = $1
        "#,
        item_name
    )
    .fetch_one(pool)
    .await
    else {
        return Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap());
    };

    Ok(item)
}

pub async fn get_columns(pool: &PgPool) -> Result<HashMap<Section, Vec<Column>>, Response<Body>> {
    let columns = query!(
        r#"
        SELECT columns.id, columns.section::text as "section!: String", columns.ordering, ARRAY[]::text[] as "crates!: Vec<Crate>" FROM columns
    "#
    ).fetch_all(pool).await;

    println!("{columns:?}");

    let mut columns = query_as!(
        Column,
        r#"
        SELECT columns.id, columns.section::text as "section!: String", columns.ordering, ARRAY[]::text[] as "crates!: Vec<Crate>" FROM columns
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(query_error_to_internal)?;

    let crates = get_crates_from_db(pool).await?;

    columns.iter_mut().for_each(|column| {
        column.crates = crates
            .iter()
            .filter(|tuple| tuple.0 == column.id)
            .cloned()
            .map(|tuple| tuple.1)
            .collect()
    });

    let mut res = HashMap::new();

    for column in columns {
        res.entry(column.section)
            .and_modify(|columns_in_map: &mut Vec<Column>| columns_in_map.push(column.clone()))
            .or_insert_with(|| vec![column]);
    }

    Ok(res)
}

pub async fn get_crates_from_db(pool: &PgPool) -> Result<Vec<(i32, Crate)>, Response<Body>> {
    let crates_flattened = query!(
        r#"
            SELECT crates.id,
                crates.rotation::text as "rotation!: String",
                crates.column_id,
                ct.name,
                ct.big_dimension,
                ct.small_dimension,
                COALESCE(ARRAY_AGG(i.*) FILTER (WHERE i.name IS NOT NULL), '{}') AS "content!: Vec<Item>"
                    FROM crates
                             LEFT JOIN crate_items ci ON crates.id = ci.crate_id
                             LEFT JOIN crate_types ct ON ct.name = crates.crate_type_name
                             LEFT JOIN items i ON ci.item_name = i.name
                    GROUP BY crates.id, crates.id, crates.rotation::text, ct.big_dimension, ct.small_dimension, ct.name
        "#).fetch_all(pool).await.map_err(query_error_to_internal)?;

    let crates = crates_flattened
        .into_iter()
        .map(|crate_record| {
            (
                crate_record.column_id,
                Crate {
                    id: crate_record.id,
                    rotation: Rotation::from(crate_record.rotation),
                    content: crate_record.content,
                    crate_type: CrateType {
                        small_dimension: crate_record.small_dimension,
                        name: crate_record.name,
                        big_dimension: crate_record.big_dimension,
                    },
                },
            )
        })
        .collect();

    Ok(crates)
}
