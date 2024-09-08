use std::collections::HashMap;

use askama::Template;

use crate::models::{Column, Crate, Item, Section};

#[derive(Template)]
#[template(path = "admin/admin.html")]
pub struct AdminTemplate {
    pub items: Vec<Item>,
    pub columns: HashMap<Section, Vec<Column>>,
}

#[derive(Template)]
#[template(path = "admin/items.html")]
pub struct ItemsTemplate {
    pub items: Vec<Item>,
}

#[derive(Template)]
#[template(path = "admin/item.html")]
pub struct ItemTemplate {
    pub item: Item,
}

#[derive(Template)]
#[template(path = "admin/columns.html")]
pub struct ColumnsTemplate {
    pub columns: HashMap<Section, Vec<Column>>,
}

#[derive(Template)]
#[template(path = "admin/crates.html")]
pub struct CratesTemplate {
    pub crates: Vec<Crate>,
}

#[derive(Template)]
#[template(path = "admin/crate.html")]
pub struct CrateTemplate {
    pub single_crate: Crate,
}
