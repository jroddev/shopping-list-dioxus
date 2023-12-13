#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use log::{info, LevelFilter};

#[cfg(feature = "ssr")]
use tokio::net::TcpListener;

mod common_types;
mod item_listing_page;
mod postgres;
mod shopping_lists_page;

use item_listing_page::ItemListingPage;
use shopping_lists_page::ShoppingListsPage;
use uuid::Uuid;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    let config = LaunchBuilder::<FullstackRouterConfig<Route>>::router();
    #[cfg(feature = "ssr")]
    {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                if let Err(e) = postgres::run_db_migrations().await {
                    panic!("Failed to run Postgres Migration: {e}");
                }

                println!("listening on http://{}", addr);

                let config =
                    ServeConfigBuilder::new_with_router(FullstackRouterConfig::<Route>::default());

                // add Axum State for Postgres connection pool
                let app = axum::Router::new()
                    .serve_dioxus_application("", config)
                    .into_make_service();
                axum::Server::bind(&addr).serve(app).await.unwrap();
            });
    }
    #[cfg(not(feature = "ssr"))]
    {
        config.launch();
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    ShoppingListsPage,
    #[route("/list/:id")]
    ItemListingPage { id: Uuid },
}
