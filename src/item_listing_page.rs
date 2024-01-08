use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::Link;
use log::{error, info};
use uuid::Uuid;

use crate::{
    common_types::{get_items, get_list, insert_new_item, Item},
    dialog_wrapper::DialogWrapper,
    Route,
};

async fn refresh_items(id: &Uuid, item_state: &UseState<Option<Vec<Item>>>) {
    match get_items(*id).await {
        Ok(data) => item_state.set(Some(data)),
        Err(e) => {
            eprintln!("Error fetching items for id {id}: {e}");
            item_state.set(None);
        }
    };
}

#[inline_props]
pub fn ItemListingPage(cx: Scope, id: Uuid) -> Element {
    let default_items: Option<Vec<Item>> = None;
    let page_name = use_state(cx, || "".to_string());
    let item_state = use_state(cx, || default_items);
    let add_dialog_open = use_state(cx, || false);
    let new_item_text = use_state(cx, || "".to_string());

    use_effect(&cx, (), |()| {
        to_owned![item_state];
        to_owned![id];
        async move {
            refresh_items(&id, &item_state).await;
        }
    });

    use_effect(&cx, (), |()| {
        to_owned![id];
        to_owned![page_name];
        async move {
            match get_list(id).await {
                Ok(data) => page_name.set(data.name),
                Err(e) => {
                    error!("error loading list: {}", e);
                }
            };
        }
    });

    match item_state.get() {
        Some(items) => {
            render! {
                div {
                    Link {
                        to: Route::ShoppingListsPage,
                        "back"
                    }
                }
                h3 { "{page_name}" }

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
                DialogWrapper {
                    is_open: add_dialog_open,
                        div {
                            "Add a new Item."
                        },
                        input {
                            placeholder: "new item",
                            onchange: |ev| {
                                new_item_text.set(ev.value.clone());
                            },
                        }
                        button {
                            onclick: |_| {
                                to_owned![id];
                                to_owned![item_state];
                                to_owned![new_item_text];
                                to_owned![add_dialog_open];
                                async move {
                                    // add the item to the db
                                    info!("insert item: {new_item_text}");
                                    match insert_new_item(id, new_item_text.to_string()).await {
                                        Ok(_) => {
                                            refresh_items(&id, &item_state).await;
                                            add_dialog_open.set(false);
                                        },
                                        Err(_) => eprintln!("Error inserting Item. Update the dialog"),
                                    }
                                }
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
