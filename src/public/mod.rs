use std::collections::HashMap;

use axum::extract::Path;

use templates::*;
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
