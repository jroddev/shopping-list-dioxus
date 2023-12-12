use dioxus::{html::dialog, prelude::*};
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
    let add_dialog_open = use_state(cx, || false);

    let refresh_items = async move {
        match get_items(*id).await {
            Ok(data) => items.set(Some(data)),
            Err(e) => todo!(),
        };
    };

    use_effect(&cx, (), |()| {
        to_owned![items];
        to_owned![id];
        async move {
            // replace this with the refresh_items lambda above
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
                h3 { "List {id}" }

                ul {
                    for item in items {
                        li {
                            id: "item-{item.id}",
                            key: "item-{item.id}",
                            item.name.clone()
                        }
                    }
                }
                button {
                    onclick: |_|{
                        add_dialog_open.set(true);
                    },
                    "+"
                }

                div {
                    position: "absolute",
                    visibility: (|| {
                        if *add_dialog_open.get() {
                            "visible"
                        } else {
                            "hidden"
                        }
                    })(),
                    pointer_events: (||{
                        if *add_dialog_open.get() {
                            "auto"
                        } else {
                            "none"
                        }
                    })(),
                    top: 0,
                    left: 0,
                    background_color: "rgba(0,0,0,0.5)",
                    width: "100%",
                    height: "100%",
                    onclick: |_|{
                        add_dialog_open.set(false);
                    },
                    "overlay"
                }
                dialog {
                    open: *add_dialog_open.get(),
                    div {
                        "Add a new Item"
                    }
                    input {
                        placeholder: "new item"
                    }
                    button {
                        onclick: |_| {
                            // add the item to the db
                            // refresh_items().await;
                            add_dialog_open.set(false);
                        },
                        "Confirm",
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
