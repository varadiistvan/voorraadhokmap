use super::{AdminHome, Blog, Builder, Home};
use dioxus::prelude::*;

//Bruh

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[nest("/admin")]
        #[route("/")]
        AdminHome {},
        #[route("/create")]
        Create {},
        #[route("/builder/:owner/:name")]
        Builder { owner: String, name: String },
    #[end_nest]
    #[route("/maps/:owner/:name")]
    Map { owner: String, name: String },
}

#[component]
fn Create() -> Element {
    todo!()
}

#[component]
fn Map(owner: String, name: String) -> Element {
    todo!()
}

#[server]
async fn create_map(owner: String, name: String) -> Result<(), ServerFnError> {
    todo!()
}
