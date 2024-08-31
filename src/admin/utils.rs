use std::error::Error;

use axum::{
    body::Body,
    http::{header, Response, StatusCode},
};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
use sqlx::query;

use crate::{models::Item, AppState};

pub fn query_error_to_internal(e: sqlx::Error) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("{e:?}")))
        .unwrap()
}

pub fn authenticate(
    auth_header: Result<TypedHeader<Authorization<Basic>>, impl Error>,
    admin_password: &str,
) -> Result<(), Response<Body>> {
    let Ok(TypedHeader(Authorization(basic))) = auth_header else {
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::WWW_AUTHENTICATE, r#"Basic realm="Admin Area""#)
            .body(Body::empty())
            .unwrap();

        return Err(response);
    };

    if basic.username() != "admin" || basic.password() != admin_password {
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::WWW_AUTHENTICATE, r#"Basic realm="Admin Area""#)
            .body(Body::empty())
            .unwrap();
        return Err(response);
    }

    Ok(())
}

pub async fn get_items(state: &AppState) -> Result<Vec<Item>, Response<Body>> {
    let items = query!(
        r#"
            SELECT
                items.name,
                COALESCE(
                    (SELECT GROUP_CONCAT(alias) 
                    FROM (SELECT alias 
                        FROM item_aliases 
                        WHERE item_aliases.item_name = items.name 
                        ORDER BY item_aliases.id)
                    ), 
                    ""
                ) AS "aliases!: String"
            FROM items
            LEFT JOIN item_aliases ON items.name = item_aliases.item_name
            GROUP BY items.name;            
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(query_error_to_internal)?;
    let items: Vec<Item> = items
        .iter()
        .map(|rec| Item {
            name: rec.name.clone(),
            aliases: if !rec.aliases.is_empty() {
                rec.aliases.split(',').map(String::from).collect()
            } else {
                vec![]
            },
        })
        .collect();
    Ok(items)
}
pub async fn get_item_by_name(item_name: &str, state: &AppState) -> Result<Item, Response<Body>> {
    let Ok(item) = query!(
        r#"
            SELECT 
                items.name,
                COALESCE(
                    (SELECT GROUP_CONCAT(alias) 
                    FROM (SELECT alias 
                        FROM item_aliases 
                        WHERE item_aliases.item_name = items.name 
                        ORDER BY item_aliases.id)
                    ), 
                    ""
                ) AS "aliases!: String"
            FROM items
            LEFT JOIN item_aliases ON items.name = item_aliases.item_name
            WHERE items.name = ?
            GROUP BY items.name
        "#,
        item_name
    )
    .fetch_one(&state.pool)
    .await
    else {
        return Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap());
    };

    Ok(Item {
        name: item.name,
        aliases: if item.aliases.is_empty() {
            vec![]
        } else {
            item.aliases.split(',').map(String::from).collect()
        },
    })
}
