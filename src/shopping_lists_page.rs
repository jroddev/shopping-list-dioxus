use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{common_types::*, dialog_wrapper::DialogWrapper, Route};
use log::{error, info};

async fn refresh_lists(list_state: &UseState<Option<Vec<List>>>) {
    match get_lists().await {
        Ok(data) => list_state.set(Some(data)),
        Err(e) => {
            eprintln!("Error fetching lists: {e}");
            list_state.set(None);
        }
    };
}

#[inline_props]
pub fn ShoppingListsPage(cx: Scope) -> Element {
    let default_list: Option<Vec<List>> = None;
    let list_state = use_state(cx, || default_list);
    let add_dialog_open = use_state(cx, || false);
    let new_list_text = use_state(cx, || "".to_string());

    use_effect(&cx, (), |()| {
        to_owned![list_state];
        async move {
            refresh_lists(&list_state).await;
        }
    });

    match list_state.get() {
        Some(lists) => {
            render! {
                style { include_str!("../src/style.css") }
                h2 { "Shopping Lists" }
                for list in lists {
                    div {
                        class: "list-item",
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
                button {
                    onclick: |_|{
                        add_dialog_open.set(true);
                    },
                    "+"
                }
                DialogWrapper {
                    is_open: add_dialog_open,
                    div {
                        "Add a new List."
                    },
                    input {
                        placeholder: "new item",
                        onchange: |ev| {
                            new_list_text.set(ev.value.clone());
                        },
                    }
                    button {
                        onclick: |_| {
                            to_owned![list_state];
                            to_owned![new_list_text];
                            to_owned![add_dialog_open];
                            async move {
                                // add the list to the db
                                info!("insert list: {new_list_text}");
                                match insert_new_list(new_list_text.to_string()).await {
                                    Ok(_) => {
                                        refresh_lists(&list_state).await;
                                        add_dialog_open.set(false);
                                    },
                                    Err(_) => eprintln!("Error inserting List. Update the dialog"),
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
