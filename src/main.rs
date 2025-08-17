#![allow(non_snake_case)]

use dioxus::prelude::*;
use uuid::Uuid;

use views::{ItemListingPage, ShoppingListsPage};

/// Common types used across the application
mod common_types;
/// Define a components module that contains all shared components for our app.
mod components;
/// PostgreSQL database operations
#[cfg(feature = "server")]
mod postgres;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MANIFEST: Asset = asset!("/assets/pwa/manifest.json");

/// The Route enum defines the structure of internal routes in our app.
#[derive(Debug, Clone, Routable, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    ShoppingListsPage {},
    #[route("/list/:id")]
    ItemListingPage { id: Uuid },
}

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

/// App is the main component of our shopping list app.
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "manifest", href: MANIFEST }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Meta { name: "theme-color", content: "#2196f3" }

        Router::<Route> {}
    }
}
