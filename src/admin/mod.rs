mod items;
use std::{collections::HashMap, error::Error, sync::Arc};

use axum::{body::Body, extract::State, http::Response};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
pub use items::*;

mod columns;
pub use columns::*;

mod templates;
use templates::*;

mod utils;
use utils::*;

use crate::{
    models::{Crate, CrateType, Rotation, Section},
    AppState,
};

pub async fn index(
    auth_header: Result<TypedHeader<Authorization<Basic>>, impl Error>,
    State(state): State<Arc<AppState>>,
) -> Result<AdminTemplate, Response<Body>> {
    authenticate(auth_header, &state.admin_password)?;

    let items = get_items(&state).await?;

    let columns = HashMap::from([(
        Section::A,
        vec![Crate {
            id: 0,
            content: vec![],
            rotation: Rotation::ShortSideWall,
            crate_type: CrateType {
                width: 30.0,
                height: 30.0,
                name: "Fake Square".into(),
            },
        }],
    )]);

    Ok(AdminTemplate {
        items: ItemsTemplate { items },
        columns: ColumnsTemplate { columns },
    })
}
