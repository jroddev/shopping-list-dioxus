use dioxus::prelude::*;

use crate::{common_types::*, components::DialogWrapper, Route};
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::Icon;
use log;
use log::info;

async fn refresh_lists(list_state: &mut Signal<Option<Vec<List>>>) {
    match get_lists().await {
        Ok(data) => *list_state.write() = Some(data),
        Err(e) => {
            eprintln!("Error fetching lists: {e}");
            *list_state.write() = None;
        }
    };
}

#[component]
pub fn EditDialog(list: List, list_state: Signal<Option<Vec<List>>>) -> Element {
    let mut edit_dialog_open = use_signal(|| false);
    let mut delete_confirmation_open = use_signal(|| false);
    let mut name_text = use_signal(|| list.name.clone());

    // Make sure that the delete confirmation dialog closes when the parent does
    use_effect(move || {
        if !*edit_dialog_open.read() {
            *delete_confirmation_open.write() = false;
        }
    });

    rsx! {
        button {
            class: "invisible-button",
            onclick: move |_| {
                *edit_dialog_open.write() = true;
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
                oninput: move |ev| {
                    *name_text.write() = ev.value();
                },
            }
            div {
                button {
                    onclick: move |_| {
                        let list_id = list.id.clone();
                        let mut list_state_clone = list_state.clone();
                        let name_text_clone = name_text.clone();
                        let mut edit_dialog_open_clone = edit_dialog_open.clone();
                        spawn(async move {
                            match update_shopping_list_name(list_id, name_text_clone.read().to_string()).await {
                                Ok(_) => {
                                    *edit_dialog_open_clone.write() = false;
                                    refresh_lists(&mut list_state_clone).await;
                                },
                                Err(_) => todo!(),
                            }
                        });
                    },
                    "Update List Name",
                }
                button {
                    onclick: move |_| {
                        *delete_confirmation_open.write() = true;
                    },
                    "Delete List",
                }
            }
            DialogWrapper {
                is_open: delete_confirmation_open,
                div { "Are you sure you want to delete '{list.name}'?" },
                button {
                    onclick: move |_| {
                        let list_id = list.id.clone();
                        *delete_confirmation_open.write() = false;
                        let mut edit_dialog_open_clone = edit_dialog_open.clone();
                        let mut list_state_clone = list_state.clone();

                        spawn(async move {
                            match delete_shopping_list(list_id).await {
                                Ok(_) => {
                                    *edit_dialog_open_clone.write() = false;
                                    refresh_lists(&mut list_state_clone).await;
                                },
                                Err(_) => todo!(),
                            }
                        });
                    },
                    "Yes",
                }
                button {
                    onclick: move |_| {
                        *delete_confirmation_open.write() = false;
                    },
                    "No"
                }
            }
        }
    }
}

#[component]
pub fn ShoppingListsPage() -> Element {
    let mut list_state = use_signal(|| None::<Vec<List>>);
    let mut new_list_text = use_signal(|| "".to_string());

    use_effect(move || {
        spawn(async move {
            // Initialize database first
            if let Err(e) = initialize_database().await {
                log::error!("Failed to initialize database: {e}");
            }
            refresh_lists(&mut list_state).await;
        });
    });

    let current_lists = list_state.read().clone();

    rsx! {
        style { {include_str!("../../assets/styling/main.css")} }
        h2 { "Shopping Lists 🛒" }

        div {
            class: "list-container",
            input {
                class: "list-item",
                placeholder: "new shopping list",
                value: "{new_list_text}",
                oninput: move |ev| {
                    *new_list_text.write() = ev.value();
                },
                onkeydown: move |ev| {
                    if ev.key() == Key::Enter {
                        let mut list_state_clone = list_state.clone();
                        let mut new_list_text_clone = new_list_text.clone();
                        spawn(async move {
                            let new_list_name = new_list_text_clone.read().clone();
                            info!("insert list: {new_list_name}");
                            match insert_new_list(new_list_name).await {
                                Ok(_) => {
                                    refresh_lists(&mut list_state_clone).await;
                                    *new_list_text_clone.write() = "".to_string();
                                },
                                Err(_) => eprintln!("Error inserting List. Update the dialog"),
                            }
                        });
                    }
                }
            }
            button {
                onclick: move |_| {
                    let mut list_state_clone = list_state.clone();
                    let mut new_list_text_clone = new_list_text.clone();
                    spawn(async move {
                        let new_list_name = new_list_text_clone.read().clone();
                        info!("insert list: {new_list_name}");
                        match insert_new_list(new_list_name).await {
                            Ok(_) => {
                                refresh_lists(&mut list_state_clone).await;
                                *new_list_text_clone.write() = "".to_string();
                            },
                            Err(_) => eprintln!("Error inserting List. Update the dialog"),
                        }
                    });
                },
                "+"
            }

            {match current_lists {
                Some(lists) => rsx! {
                    {lists.iter().map(|list| {
                        let list_id = list.id;
                        let list_name = list.name.clone();
                        let list_clone = list.clone();

                        rsx! {
                            div {
                                key: "list-{list_id}",
                                class: "list-item",
                                Link {
                                    to: Route::ItemListingPage {
                                        id: list_id,
                                    },
                                    {list_name}
                                }
                                EditDialog {
                                    list: list_clone,
                                    list_state
                                }
                            }
                        }
                    })}
                },
                None => rsx! { div { "Loading..." } }
            }}
        }
    }
}
