use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::Icon;
use dioxus_router::prelude::Link;
use log::{error, info};
use uuid::Uuid;

use crate::{
    common_types::{
        clear_all_crossed, get_items, get_list, insert_new_item, update_item_crossed, Item,
    },
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

    let crossed_class = |crossed: bool| {
        if crossed {
            "crossed"
        } else {
            ""
        }
    };

    match item_state.get() {
        Some(items) => {
            render! {
                style { include_str!("../src/style.css") }
                div {
                    class: "item-page-header",
                    Link {
                        to: Route::ShoppingListsPage,
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "black",
                            icon: BsHouse,
                        }
                    }
                    h2 { "{page_name}" }
                }

                div {
                    class: "list-container",
                    input {
                        class: "list-item",
                        placeholder: "new item",
                        value: "{new_item_text}",
                        oninput: |ev| {
                            new_item_text.set(ev.value.clone());
                        },
                        onkeypress: |ev| {
                            to_owned![id];
                            to_owned![item_state];
                            to_owned![new_item_text];
                            async move {
                                // Enter not working on mobile browser with virtual keyboard.
                                // It looks like this may be fixed in dioxus 0.5
                                if ev.key() == Key::Enter {
                                    info!("insert item: {new_item_text}");
                                    match insert_new_item(id, new_item_text.current().to_string()).await {
                                        Ok(_) => {
                                            refresh_items(&id, &item_state).await;
                                            new_item_text.set("".to_string());
                                        },
                                        Err(_) => eprintln!("Error inserting Item. Update the dialog"),
                                    }
                                }
                            }
                        }
                    }
                    // Enter not working on mobile browser with virtual keyboard.
                    // It looks like this may be fixed in dioxus 0.5
                    // for now just have an extra button
                    button {
                        onclick: |_| {

                            to_owned![id];
                            to_owned![item_state];
                            to_owned![new_item_text];
                            async move {
                                info!("insert item: {new_item_text}");
                                match insert_new_item(id, new_item_text.current().to_string()).await {
                                    Ok(_) => {
                                        refresh_items(&id, &item_state).await;
                                        new_item_text.set("".to_string());
                                    },
                                    Err(_) => eprintln!("Error inserting Item. Update the dialog"),
                                }
                            }
                        },
                        "+"
                    }
                    for item in items {
                        // let crossed_style = "";// if item.crossed { "crossed "} else { "" }

                        div {
                            id: "item-{item.id}",
                            key: "item-{item.id}",
                            class: "list-item {crossed_class(item.crossed)}",
                            onclick: |_| {
                                to_owned![id];
                                to_owned![item];
                                to_owned![item_state];
                                async move {
                                    // toggle crossed
                                    match update_item_crossed(item.id, !item.crossed).await {
                                        Ok(()) => { refresh_items(&id, &item_state).await; }
                                        Err(e) => {
                                            error!("Error updating crossed state of item {}: {e}", item.id);
                                        }
                                    }
                                }
                            },
                            item.name.clone()
                        }
                    }
                    button {
                        class: "list-item",
                        onclick: |_|{
                            to_owned![id];
                            to_owned![item_state];
                            async move {
                                let _ = clear_all_crossed(id).await;
                                refresh_items(&id, &item_state).await;

                            }
                        },
                        "Delete crossed",
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "black",
                            icon: BsTrash3Fill,
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
