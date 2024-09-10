use std::{collections::HashMap, sync::Arc};

use axum::{extract::Path, routing::get, Router};

use templates::*;

use crate::AppState;
mod templates;

pub async fn index() -> MainTemplate {
    let crates = get_crates(Path("".into())).await;

    MainTemplate {
        crates,
        search_terms: vec![
            ("Coke".into(), vec!["coca cola".into()]),
            ("Fanta".into(), vec!["sinaas".into()]),
        ],
    }
}

pub async fn get_crates(Path(term): Path<String>) -> CratesTemplate {
    CratesTemplate {
        sections: HashMap::from([(
            "a".into(),
            vec![CrateTemplate {
                x: 0.0,
                y: 0.0,
                width: 30.0,
                height: 30.0,
                color: if term == "Fanta" {
                    "red".into()
                } else {
                    "blue".into()
                },
            }],
        )]),
    }
}

pub fn get_public_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index))
        .route("/rectangles/:term", get(get_crates))
}
