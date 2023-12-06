use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use uuid::Uuid;

use crate::{
    common_types::{get_items, Item},
    Route,
};

#[inline_props]
pub fn ItemListingPage(cx: Scope, id: Uuid) -> Element {
    let default_items: Option<Vec<Item>> = None;
    let items = use_state(cx, || default_items);

    use_effect(&cx, (), |()| {
        to_owned![items];
        to_owned![id];
        async move {
            match get_items(id).await {
                Ok(data) => items.set(Some(data)),
                Err(e) => todo!(),
            };
        }
    });

    match items.get() {
        Some(items) => {
            render! {
                div {
                    Link {
                        to: Route::ShoppingListsPage,
                        "back"
                    }
                }
                h3 { "list {id}" }

                ul {
                    for item in items {
                        li {
                            id: "item-{item.id}",
                            key: "item-{item.id}",
                            item.name.clone()
                        }
                    }
                }
            }
        }
        None => {
            render! {
                "Loading.."
            }
        }
    }
}
