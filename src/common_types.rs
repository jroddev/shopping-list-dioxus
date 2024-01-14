use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::postgres::{self};

#[server(ServerLog)]
pub async fn server_log(text: String) -> Result<(), ServerFnError> {
    info!("server_log: {text}");
    Ok(())
}

#[server(InsertNewItem)]
pub async fn insert_new_item(id: Uuid, new_item_text: String) -> Result<(), ServerFnError> {
    match postgres::create_list_item(&new_item_text, id).await {
        Ok(row) => {
            info!("Inserted: {row:?}");
            Ok(())
        }
        Err(e) => {
            error!("Error inserting item {new_item_text} in list {id}: {e}");
            Err(ServerFnError::ServerError("Failed to add item".to_string()))
        }
    }
}

#[server(UpdateItemCrossed)]
pub async fn update_item_crossed(id: Uuid, crossed: bool) -> Result<(), ServerFnError> {
    match postgres::update_item_crossed(id, crossed).await {
        Ok(row) => {
            info!("Inserted: {row:?}");
            Ok(())
        }
        Err(e) => {
            error!("Error updating crossed state for item {id}: {e}");
            Err(ServerFnError::ServerError("Failed to add item".to_string()))
        }
    }
}

#[server(ClearAllCrossed)]
pub async fn clear_all_crossed(list_id: Uuid) -> Result<(), ServerFnError> {
    match postgres::clear_all_crossed(list_id).await {
        Ok(()) => {
            info!("Deleted crossed items from list {list_id}");
            Ok(())
        }
        Err(e) => {
            error!("Error deleting crossed from list {list_id}: {e}");
            Err(ServerFnError::ServerError(
                "Failed to clear crossed items".to_string(),
            ))
        }
    }
}

#[server(InsertNewList)]
pub async fn insert_new_list(new_list_text: String) -> Result<(), ServerFnError> {
    match postgres::create_shopping_list(&new_list_text).await {
        Ok(row) => {
            info!("Inserted: {row:?}");
            Ok(())
        }
        Err(e) => {
            error!("Error inserting list {new_list_text}: {e}");
            Err(ServerFnError::ServerError("Failed to add list".to_string()))
        }
    }
}

#[server(DeleteShoppingList)]
pub async fn delete_shopping_list(id: Uuid) -> Result<(), ServerFnError> {
    match postgres::delete_shopping_list(id).await {
        Ok(()) => {
            info!("Deleted Shopping List with ID: {id}");
            Ok(())
        }
        Err(e) => {
            error!("Error deleting list {id}: {e}");
            Err(ServerFnError::ServerError(
                "Failed to delete list".to_string(),
            ))
        }
    }
}

#[server(GetItems)]
pub async fn get_items(list_id: Uuid) -> Result<Vec<Item>, ServerFnError> {
    println!("get items: {list_id}");
    match postgres::get_list_items(list_id).await {
        Ok(items) => {
            println!("got the items: {:?}", items);
            Ok(items)
        }
        Err(e) => {
            eprintln!("error grabbing items: {:?}", e);
            Err(ServerFnError::ServerError(
                "Could not retrieve Items from Database".to_string(),
            ))
        }
    }
}

#[server(GetList)]
pub async fn get_list(id: Uuid) -> Result<List, ServerFnError> {
    println!("get_list");

    match postgres::get_shopping_list(id).await {
        Ok(list) => {
            println!("got the list: {:?}", list);
            Ok(list)
        }
        Err(e) => {
            eprintln!("error grabbing list: {:?}", e);
            Err(ServerFnError::ServerError(
                "Could not retrieve Lists from Database".to_string(),
            ))
        }
    }
}

#[server(GetLists)]
pub async fn get_lists() -> Result<Vec<List>, ServerFnError> {
    println!("get_lists");

    // println!(
    //     "insert: {:?}",
    //     postgres::create_shopping_list("Woolies").await
    // );

    match postgres::get_shopping_lists().await {
        Ok(lists) => {
            println!("got the lists: {:?}", lists);

            // postgres::create_list_item("toilet paper", lists.get(0).unwrap().id).await;

            Ok(lists)
        }
        Err(e) => {
            eprintln!("error grabbing lists: {:?}", e);
            Err(ServerFnError::ServerError(
                "Could not retrieve Lists from Database".to_string(),
            ))
        }
    }

    // Ok(vec![
    // List::new(
    //     "Woolies",
    //     &[Item::new("cheese"), Item::new("milk"), Item::new("apples")],
    // ),
    // List::new("Aldi", &[Item::new("Feta")]),
    // List::new(
    //     "CostCo",
    //     &[
    //         Item::new("carrots"),
    //         Item::new("steak"),
    //         Item::new("mixed nuts"),
    //     ],
    // ),
    // ])
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub crossed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// impl Item {
//     pub fn new(name: &str) -> Self {
//         Item {
//             id: Uuid::new_v4(),
//             name: name.to_string(),
//             crossed: false,
//             created_at: Utc::now(),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// impl List {
//     pub fn new(name: &str, items: &[Item]) -> Self {
//         List {
//             id: Uuid::new_v4(),
//             name: name.to_string(),
//             items: items.to_vec(),
//             created_at: Utc::now(),
//         }
//     }
// }
