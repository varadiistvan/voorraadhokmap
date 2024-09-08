mod items;
use std::{collections::HashMap, error::Error, sync::Arc};

use axum::{body::Body, extract::State, http::Response};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
use fake::{faker, Fake, Faker};
pub use items::*;

mod columns;
pub use columns::*;

mod templates;
use templates::*;

mod utils;
use utils::*;

use crate::{
    models::{Column, Crate, CrateType, Rotation, Section},
    AppState,
};

pub async fn index(
    auth_header: Result<TypedHeader<Authorization<Basic>>, impl Error>,
    State(state): State<Arc<AppState>>,
) -> Result<AdminTemplate, Response<Body>> {
    authenticate(auth_header, &state.admin_password)?;

    let items = get_items(&state).await?;

    let columns = HashMap::from([
        (
            Section::A,
            vec![Faker.fake(), Faker.fake(), Faker.fake(), Faker.fake()],
        ),
        (Section::B, vec![Faker.fake(), Faker.fake()]),
        (Section::C, vec![]),
        (Section::D, vec![]),
        (Section::E, vec![]),
    ]);

    println!("{columns:?}");

    Ok(AdminTemplate { items, columns })
}
