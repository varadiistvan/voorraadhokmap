use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Type, Hash)]
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
pub struct Column {
    pub id: usize,
    pub section: Section,
    pub boxes: Vec<Crate>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Type)]
#[sqlx(type_name = "TEXT")]
pub enum Rotation {
    LongSideWall,
    ShortSideWall,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Crate {
    pub id: usize,
    pub rotation: Rotation,
    pub content: Vec<Item>,
    pub crate_type: CrateType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CrateType {
    pub name: String,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Type)]
pub struct Item {
    pub name: String,
    pub aliases: Vec<String>,
}
