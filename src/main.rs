use dioxus::prelude::*;
use router::Route;

#[cfg(feature = "server")]
mod entity;
mod models;
mod router;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[server]
async fn get_crates() -> Result<Vec<entity::crates::Model>, ServerFnError> {
    todo!()
}

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        "hoi"
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            "Bruv {id}"
        }
    }
}

#[component]
pub fn AdminHome() -> Element {
    rsx! {
        "Admin innit"
    }
}
#[component]
pub fn Builder(owner: String, name: String) -> Element {
    rsx! {
        "{owner}: {name}"
    }
}
