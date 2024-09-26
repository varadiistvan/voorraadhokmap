use fake::Dummy;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Hash)]
#[cfg_attr(debug_assertions, derive(Dummy))]
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

impl From<String> for Section {
    fn from(value: String) -> Self {
        match value.as_str() {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            "G" => Self::G,
            "H" => Self::H,
            _ => panic!("Unsupported string for section: {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Dummy))]
pub struct Column {
    pub id: i32,
    pub section: Section,
    pub crates: Vec<Crate>,
    pub ordering: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Type)]
#[cfg_attr(debug_assertions, derive(Dummy))]
pub enum Rotation {
    LongSideWall,
    ShortSideWall,
}

impl From<String> for Rotation {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ShortSideWall" => Self::ShortSideWall,
            "LongSideWall" => Self::LongSideWall,
            _ => panic!("Unsupported string {value} for rotation"),
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShortSideWall => f.write_str("Short Side Wall"),
            Self::LongSideWall => f.write_str("Long Side Wall"),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Type)]
#[cfg_attr(debug_assertions, derive(Dummy))]
pub struct Crate {
    #[dummy(faker = "0..20")]
    pub id: i32,
    pub rotation: Rotation,
    pub content: Vec<Item>,
    pub crate_type: CrateType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Type)]
#[cfg_attr(debug_assertions, derive(Dummy))]
pub struct CrateType {
    pub name: String,
    pub small_dimension: f32,
    pub big_dimension: f32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Type)]
#[cfg_attr(debug_assertions, derive(Dummy))]
pub struct Item {
    pub name: String,
    pub aliases: Vec<String>,
}
