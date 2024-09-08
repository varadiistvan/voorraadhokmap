use std::fmt::Display;

use fake::Dummy;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Type, Hash)]
#[cfg_attr(debug_assertions, derive(Dummy))]
#[sqlx(type_name = "TEXT")]
pub enum Section {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Dummy))]
pub struct Column {
    pub id: usize,
    pub section: Section,
    pub crates: Vec<Crate>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Type)]
#[cfg_attr(debug_assertions, derive(Dummy))]
#[sqlx(type_name = "TEXT")]
pub enum Rotation {
    LongSideWall,
    ShortSideWall,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShortSideWall => f.write_str("Short Side Wall"),
            Self::LongSideWall => f.write_str("Long Side Wall"),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Dummy))]
pub struct Crate {
    #[dummy(faker = "0..20")]
    pub id: usize,
    pub rotation: Rotation,
    pub content: Vec<Item>,
    pub crate_type: CrateType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Dummy))]
pub struct CrateType {
    pub name: String,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Type)]
#[cfg_attr(debug_assertions, derive(Dummy))]
pub struct Item {
    pub name: String,
    pub aliases: Vec<String>,
}
