use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::Icon;
use dioxus_router::prelude::Link;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    common_types::{
        clear_all_crossed, get_items, get_list, insert_new_item, update_item_crossed, Item,
    },
    Route,
};

async fn refresh_items(id: &Uuid, item_state: &mut SyncSignal<Option<Vec<Item>>>) {
    match get_items(*id).await {
        Ok(data) => item_state.set(Some(data)),
        Err(e) => {
            eprintln!("Error fetching items for id {id}: {e}");
            item_state.set(None);
        }
    };
}

#[component]
pub fn ItemListingPage(id: Uuid) -> Element {
    let default_items: Option<Vec<Item>> = None;
    let mut page_name = use_signal(|| "".to_string());
    let mut item_state = use_signal_sync(|| default_items);
    let mut add_dialog_open = use_signal(|| false);
    let mut new_item_text = use_signal(|| "".to_string());

    use_effect(move || {
        spawn(async move {
            refresh_items(&id, &mut item_state).await;
        });
    });

    use_effect(move || {
        spawn(async move {
            match get_list(id).await {
                Ok(data) => page_name.set(data.name),
                Err(e) => {
                    error!("error loading list: {}", e);
                }
            };
        });
    });

    let crossed_class = |crossed: bool| {
        if crossed {
            "crossed"
        } else {
            ""
        }
    };

    match item_state() {
        Some(items) => {
            render! {
                style { {include_str!("./assets/style.css")} }
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
                        oninput: move |ev| {
                            new_item_text.set(ev.value().clone());
                        },
                        onkeypress: move |ev| {
                            async move {
                                if ev.key() == Key::Enter {
                                    info!("insert item: {new_item_text}");
                                    match insert_new_item(id, new_item_text().to_string()).await {
                                        Ok(_) => {
                                            refresh_items(&id, &mut item_state).await;
                                            new_item_text.set("".to_string());
                                        },
                                        Err(_) => eprintln!("Error inserting Item. Update the dialog"),
                                    }
                                }
                            }
                        }
                    }
                    for item in items {
                        div {
                            id: "item-{item.id}",
                            key: "item-{item.id}",
                            class: "list-item {crossed_class(item.crossed)}",
                            onclick: move |_| {
                                async move {
                                    // toggle crossed
                                    match update_item_crossed(item.id, !item.crossed).await {
                                        Ok(()) => { refresh_items(&id, &mut item_state).await; }
                                        Err(e) => {
                                            error!("Error updating crossed state of item {}: {e}", item.id);
                                        }
                                    }
                                }
                            },
                            { item.name.clone() }
                        }
                    }
                    button {
                        class: "list-item",
                        onclick: move |_|{
                            async move {
                                let _ = clear_all_crossed(id).await;
                                refresh_items(&id, &mut item_state).await;

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
