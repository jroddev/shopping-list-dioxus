use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::Icon;
use log::{error, info};
use uuid::Uuid;

use crate::{
    common_types::{
        clear_all_crossed, get_items, get_list, insert_new_item, update_item_crossed, Item,
    },
    Route,
};

async fn refresh_items(id: &Uuid, item_state: &mut Signal<Option<Vec<Item>>>) {
    match get_items(*id).await {
        Ok(data) => *item_state.write() = Some(data),
        Err(e) => {
            eprintln!("Error fetching items for id {id}: {e}");
            *item_state.write() = None;
        }
    };
}

#[component]
pub fn ItemListingPage(id: Uuid) -> Element {
    let mut page_name = use_signal(|| "".to_string());
    let mut item_state = use_signal(|| None::<Vec<Item>>);
    let mut new_item_text = use_signal(|| "".to_string());

    use_effect(move || {
        spawn(async move {
            refresh_items(&id, &mut item_state).await;
        });
    });

    use_effect(move || {
        spawn(async move {
            match get_list(id).await {
                Ok(data) => *page_name.write() = data.name,
                Err(e) => {
                    error!("error loading list: {}", e);
                }
            }
        });
    });

    let current_items = item_state.read().clone();

    let crossed_class = |crossed: bool| {
        if crossed {
            "crossed"
        } else {
            ""
        }
    };

    match current_items {
        Some(items) => rsx! {
            style { {include_str!("../../assets/styling/main.css")} }
            div {
                class: "item-page-header",
                Link {
                    to: Route::ShoppingListsPage {},
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
                        *new_item_text.write() = ev.value();
                    },
                    onkeydown: move |ev| {
                        if ev.key() == Key::Enter {
                            let mut item_state_clone = item_state.clone();
                            let mut new_item_text_clone = new_item_text.clone();
                            spawn(async move {
                                let new_item_name = new_item_text_clone.read().clone();
                                info!("insert item: {new_item_name}");
                                match insert_new_item(id, new_item_name).await {
                                    Ok(_) => {
                                        refresh_items(&id, &mut item_state_clone).await;
                                        *new_item_text_clone.write() = "".to_string();
                                    },
                                    Err(_) => eprintln!("Error inserting Item. Update the dialog"),
                                }
                            });
                        }
                    }
                }
                // button {
                //     onclick: move |_| {
                //         let mut item_state_clone = item_state.clone();
                //         let mut new_item_text_clone = new_item_text.clone();
                //         spawn(async move {
                //             let new_item_name = new_item_text_clone.read().clone();
                //             info!("insert item: {new_item_name}");
                //             match insert_new_item(id, new_item_name).await {
                //                 Ok(_) => {
                //                     refresh_items(&id, &mut item_state_clone).await;
                //                     *new_item_text_clone.write() = "".to_string();
                //                 },
                //                 Err(_) => eprintln!("Error inserting Item. Update the dialog"),
                //             }
                //         });
                //     },
                //     "+"
                // }

                {items.iter().map(|item| {
                    let item_id = item.id;
                    let item_name = item.name.clone();
                    let is_crossed = item.crossed;

                    rsx! {
                        div {
                            key: "item-{item_id}",
                            id: "item-{item_id}",
                            class: "list-item {crossed_class(is_crossed)}",
                            onclick: move |_| {
                                let mut item_state_clone = item_state.clone();
                                let new_crossed = !is_crossed;
                                spawn(async move {
                                    match update_item_crossed(item_id, new_crossed).await {
                                        Ok(_) => {
                                            refresh_items(&id, &mut item_state_clone).await;
                                        },
                                        Err(e) => {
                                            error!("Error updating crossed state of item {}: {e}", item_id);
                                        }
                                    }
                                });
                            },
                            {item_name}
                        }
                    }
                })}

                button {
                    class: "list-item",
                    onclick: move |_| {
                        let mut item_state_clone = item_state.clone();
                        spawn(async move {
                            match clear_all_crossed(id).await {
                                Ok(_) => {
                                    refresh_items(&id, &mut item_state_clone).await;
                                },
                                Err(e) => eprintln!("Error clearing crossed items: {e}"),
                            }
                        });
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
        },
        None => rsx! {
            style { {include_str!("../../assets/styling/main.css")} }
            div {
                class: "item-page-header",
                Link {
                    to: Route::ShoppingListsPage {},
                    Icon {
                        width: 30,
                        height: 30,
                        fill: "black",
                        icon: BsHouse,
                    }
                }
                h2 { "{page_name}" }
            }
            div { "Loading..." }
        },
    }
}
