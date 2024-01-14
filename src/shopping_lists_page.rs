use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_router::prelude::Link;

use crate::{common_types::*, dialog_wrapper::DialogWrapper, Route};
use log::info;

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
pub fn EditDialog(cx: Scope, list: List, list_state: UseState<Option<Vec<List>>>) -> Element {
    let edit_dialog_open = use_state(cx, || false);
    let delete_confirmation_open = use_state(cx, || false);
    let name_text = use_state(cx, || list.name.clone());

    // Make sure that the delete confirmation dialog closes when the parent does
    use_effect(&cx, &(edit_dialog_open.clone(),), |_edit_dialog_open| {
        to_owned![delete_confirmation_open];
        async move {
            delete_confirmation_open.set(false);
        }
    });

    render! {
        button {
            onclick: |_| {
                edit_dialog_open.set(true);
            },
            "E"
        }
        DialogWrapper {
            is_open: edit_dialog_open,
            div {
                "Edit Shopping List Name",
            }
            input {
                value: "{name_text}",
                onchange: |ev| {
                    name_text.set(ev.value.clone());
                },
            }
            button {
                onclick: |_| { delete_confirmation_open.set(true); },
                "Delete List",
            }
            DialogWrapper {
                is_open: delete_confirmation_open,
                div { "Are you sure you want to delete '{list.name}'?" },
                button {
                    onclick: |_| {
                        let list_id = list.id.clone();
                        delete_confirmation_open.set(false);
                        to_owned![edit_dialog_open, list_state, list];

                        async move {
                            match delete_shopping_list(list_id).await {
                                Ok(_) => {
                                    edit_dialog_open.set(false);
                                    list_state.modify(|state|{
                                            match state.clone() {
                                                Some(state) =>  {
                                                    Some(state.iter()
                                                        .filter(|x| x.id != list.id)
                                                        .map(|x| x.to_owned())
                                                        .collect())
                                                },
                                                None => None,
                                            }
                                    });
                                },
                                Err(_) => todo!(),
                            }
                        }
                    },
                    "Yes",
                }
                button {
                    onclick: |_| { delete_confirmation_open.set(false); },
                    "No"
                }
            }
        }
    }
}

#[inline_props]
pub fn ShoppingListsPage(cx: Scope) -> Element {
    let default_list: Option<Vec<List>> = None;
    let list_state = use_state(cx, || default_list);
    // let add_dialog_open = use_state(cx, || false);
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
                div {
                    class: "list-container",
                    input {
                        class: "list-item",
                        placeholder: "new shopping list",
                        value: "{new_list_text}",
                        oninput: |ev| {
                            new_list_text.set(ev.value.clone());
                        },
                        onkeypress: |ev| {
                            to_owned![new_list_text, list_state];
                            async move {
                                // Enter not working on mobile browser with virtual keyboard.
                                // Diff key?
                                // info!("Key Press: {}", ev.key());
                                if ev.key() == Key::Enter {
                                    let new_list_name = new_list_text.current();
                                    info!("insert list: {new_list_name}");
                                    match insert_new_list(new_list_name.to_string()).await {
                                        Ok(_) => {
                                            refresh_lists(&list_state).await;
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
                                list.name.clone()
                            }
                            EditDialog {
                                list: list.clone(),
                                list_state: list_state.clone()
                            }
                        }
                    }
                }
                // button {
                //     onclick: |_|{
                //         add_dialog_open.set(true);
                //     },
                //     "+"
                // }
                // DialogWrapper {
                //     is_open: add_dialog_open,
                //     div {
                //         "Add a new List."
                //     },
                //     input {
                //         placeholder: "new item",
                //         onchange: |ev| {
                //             new_list_text.set(ev.value.clone());
                //         },
                //     }
                //     button {
                //         onclick: |_| {
                //             to_owned![list_state];
                //             to_owned![new_list_text];
                //             to_owned![add_dialog_open];
                //             async move {
                //                 // add the list to the db
                //                 info!("insert list: {new_list_text}");
                //                 match insert_new_list(new_list_text.to_string()).await {
                //                     Ok(_) => {
                //                         refresh_lists(&list_state).await;
                //                         add_dialog_open.set(false);
                //                     },
                //                     Err(_) => eprintln!("Error inserting List. Update the dialog"),
                //                 }
                //             }
                //         },
                //         "Confirm",
                //     }
                // }
            }
        }
        None => {
            render! {
                "Loading.."
            }
        }
    }
}
