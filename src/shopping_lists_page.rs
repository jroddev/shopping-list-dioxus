use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{common_types::*, Route};

#[inline_props]
pub fn ShoppingListsPage(cx: Scope) -> Element {
    let default_list: Option<Vec<List>> = None;
    let lists = use_state(cx, || default_list);

    use_effect(&cx, (), |()| {
        to_owned![lists];
        async move {
            match get_lists().await {
                Ok(data) => lists.set(Some(data)),
                Err(e) => todo!(),
            };
        }
    });

    match lists.get() {
        Some(lists) => {
            render! {
                h2 { "Shopping Lists" }
                for list in lists {
                    div {
                        Link {
                            id: "list-{list.id}",
                            key: "list-{list.id}",
                            to: Route::ItemListingPage {
                                id: list.id,
                            },
                            list.name.clone()
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
