use std::collections::HashMap;

use askama::Template;

#[derive(Template)]
#[template(path = "public/main.html")]
pub struct MainTemplate {
    pub crates: CratesTemplate,
    pub search_terms: Vec<(String, Vec<String>)>,
}

#[derive(Template)]
#[template(path = "public/crates.html")]
pub struct CratesTemplate {
    pub sections: HashMap<String, Vec<CrateTemplate>>,
}

#[derive(Template)]
#[template(path = "public/crate.html")]
pub struct CrateTemplate {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub color: String,
}
