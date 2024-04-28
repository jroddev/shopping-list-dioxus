use dioxus::html::i;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

use crate::{common_types::*, dialog_wrapper::DialogWrapper, Route};
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::Icon;
use tracing::info;

async fn refresh_lists(list_state: &mut SyncSignal<Option<Vec<List>>>) {
    match get_lists().await {
        Ok(data) => list_state.set(Some(data)),
        Err(e) => {
            eprintln!("Error fetching lists: {e}");
            list_state.set(None);
        }
    };
}

#[component]
pub fn EditDialog(list: List, list_state: SyncSignal<Option<Vec<List>>>) -> Element {
    let mut edit_dialog_open = use_signal_sync(|| false);
    let mut delete_confirmation_open = use_signal_sync(|| false);
    let mut name_text = use_signal(|| list.name.clone());

    use_effect(move || {
        spawn(async move {
            delete_confirmation_open.set(false);
        });
    });

    rsx! {
        button {
            class: "invisible-button",
            onclick: move |_| {
                edit_dialog_open.set(true);
            },
            Icon {
                width: 30,
                height: 30,
                fill: "black",
                icon: BsPencil,
            }
        }
        DialogWrapper {
            is_open: edit_dialog_open,
            div {
                "Edit Shopping List Name",
            }
            input {
                value: "{name_text}",
                onchange: move |ev| {
                    name_text.set(ev.value().clone());
                },
            }
            div {
                button {
                    onclick:  move|_| {
                        async move {
                            match update_shopping_list_name(list.id, name_text.to_string()).await {
                                Ok(_) => {
                                    edit_dialog_open.set(false);
                                    refresh_lists(&mut list_state).await;
                                },
                                Err(_) => todo!(),
                            }
                        }
                    },
                    "Update List Name",
                }
                button {
                    onclick: move |_| { delete_confirmation_open.set(true); },
                    "Delete List",
                }
            }
            DialogWrapper {
                is_open: delete_confirmation_open,
                div { "Are you sure you want to delete '{list.name}'?" },
                button {
                    onclick: move |_| {
                        delete_confirmation_open.set(false);

                        async move {
                            match delete_shopping_list(list.id).await {
                                Ok(_) => {
                                    edit_dialog_open.set(false);
                                    refresh_lists(&mut list_state).await;
                                },
                                Err(_) => todo!(),
                            }
                        }
                    },
                    "Yes",
                }
                button {
                    onclick: move |_| { delete_confirmation_open.set(false); },
                    "No"
                }
            }
        }
    }
}

#[component]
pub fn ShoppingListsPage() -> Element {
    let default_list: Option<Vec<List>> = None;
    let mut list_state = use_signal_sync(|| default_list);
    let mut new_list_text = use_signal(|| "".to_string());

    use_effect(move || {
        spawn(async move {
            refresh_lists(&mut list_state).await;
        });
    });

    match list_state() {
        Some(lists) => {
            rsx! {
                style { {include_str!("./assets/style.css")} }
                h2 { "Shopping Lists" }
                div {
                    class: "list-container",
                    input {
                        class: "list-item",
                        placeholder: "new shopping list",
                        value: "{new_list_text}",
                        oninput: move |ev| {
                            new_list_text.set(ev.value().clone());
                        },
                        onkeypress: move |ev| {
                            async move {
                                if ev.key() == Key::Enter {
                                    let new_list_name = new_list_text();
                                    info!("insert list: {new_list_name}");
                                    match insert_new_list(new_list_name.to_string()).await {
                                        Ok(_) => {
                                            refresh_lists(&mut list_state).await;
                                            new_list_text.set("".to_string());
                                        },
                                        Err(_) => eprintln!("Error inserting List. Update the dialog"),
                                    }
                                }
                            }
                        }
                    }
                    for list in lists {
                        div {
                            id: "list-{list.id}",
                            key: "list-{list.id}",
                            class: "list-item",
                            Link {
                                to: Route::ItemListingPage {
                                    id: list.id,
                                },
                                { list.name.clone() }
                            }
                            EditDialog {
                                list: list.clone(),
                                list_state: list_state
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                "Loading.."
            }
        }
    }
}
