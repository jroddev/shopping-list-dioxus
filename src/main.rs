#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod common_types;
mod dialog_wrapper;
mod item_listing_page;
mod postgres;
mod shopping_lists_page;
//
use item_listing_page::ItemListingPage;
use shopping_lists_page::ShoppingListsPage;
use uuid::Uuid;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let mut config = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                if let Err(e) = postgres::run_db_migrations().await {
                    panic!("Failed to run Postgres Migration: {e}");
                }
            });

        config = config.addr(std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::new(0, 0, 0, 0),
            8080,
        ));
    }

    LaunchBuilder::new().with_cfg(config).launch(App)
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    ShoppingListsPage,
    #[route("/list/:id")]
    ItemListingPage { id: Uuid },
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
