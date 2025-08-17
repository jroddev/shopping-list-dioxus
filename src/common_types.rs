use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Server functions let us define public APIs on the server that can be called like a normal async function from the client.
// Each server function needs to be annotated with the `#[server]` attribute, accept and return serializable types, and return
// a `Result` with the error type [`ServerFnError`].
//
// When the server function is called from the client, it will just serialize the arguments, call the API, and deserialize the
// response.

#[server(InitializeDatabase)]
pub async fn initialize_database() -> Result<(), ServerFnError> {
    use crate::postgres;
    match postgres::run_db_migrations().await {
        Ok(_) => {
            log::info!("Database migrations completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Database migration failed: {e}");
            Err(ServerFnError::ServerError(
                "Failed to initialize database".to_string(),
            ))
        }
    }
}

#[server(ServerLog)]
pub async fn server_log(text: String) -> Result<(), ServerFnError> {
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.
    log::info!("server_log: {text}");
    Ok(())
}

#[server(InsertNewItem)]
pub async fn insert_new_item(id: Uuid, new_item_text: String) -> Result<(), ServerFnError> {
    use crate::postgres;
    match postgres::create_list_item(&new_item_text, id).await {
        Ok(row) => {
            log::info!("Inserted: {row:?}");
            Ok(())
        }
        Err(e) => {
            log::error!("Error inserting item {new_item_text} in list {id}: {e}");
            Err(ServerFnError::ServerError("Failed to add item".to_string()))
        }
    }
}

#[server(UpdateItemCrossed)]
pub async fn update_item_crossed(id: Uuid, crossed: bool) -> Result<(), ServerFnError> {
    use crate::postgres;
    match postgres::update_item_crossed(id, crossed).await {
        Ok(row) => {
            log::info!("Updated: {row:?}");
            Ok(())
        }
        Err(e) => {
            log::error!("Error updating crossed state for item {id}: {e}");
            Err(ServerFnError::ServerError(
                "Failed to update item".to_string(),
            ))
        }
    }
}

#[server(ClearAllCrossed)]
pub async fn clear_all_crossed(list_id: Uuid) -> Result<(), ServerFnError> {
    use crate::postgres;
    match postgres::clear_all_crossed(list_id).await {
        Ok(()) => {
            log::info!("Deleted crossed items from list {list_id}");
            Ok(())
        }
        Err(e) => {
            log::error!("Error deleting crossed from list {list_id}: {e}");
            Err(ServerFnError::ServerError(
                "Failed to clear crossed items".to_string(),
            ))
        }
    }
}

#[server(InsertNewList)]
pub async fn insert_new_list(new_list_text: String) -> Result<(), ServerFnError> {
    use crate::postgres;
    match postgres::create_shopping_list(&new_list_text).await {
        Ok(row) => {
            log::info!("Inserted: {row:?}");
            Ok(())
        }
        Err(e) => {
            log::error!("Error inserting list {new_list_text}: {e}");
            Err(ServerFnError::ServerError("Failed to add list".to_string()))
        }
    }
}

#[server(DeleteShoppingList)]
pub async fn delete_shopping_list(id: Uuid) -> Result<(), ServerFnError> {
    use crate::postgres;
    match postgres::delete_shopping_list(id).await {
        Ok(()) => {
            log::info!("Deleted Shopping List with ID: {id}");
            Ok(())
        }
        Err(e) => {
            log::error!("Error deleting list {id}: {e}");
            Err(ServerFnError::ServerError(
                "Failed to delete list".to_string(),
            ))
        }
    }
}

#[server(UpdateShoppingListName)]
pub async fn update_shopping_list_name(id: Uuid, new_name: String) -> Result<(), ServerFnError> {
    use crate::postgres;
    match postgres::update_shopping_list_name(id, new_name.clone()).await {
        Ok(()) => {
            log::info!("Update Shopping List Name with ID: {id} to {new_name}");
            Ok(())
        }
        Err(e) => {
            log::error!("Error updating list name {id}: {e}");
            Err(ServerFnError::ServerError(
                "Failed to update list name".to_string(),
            ))
        }
    }
}

#[server(GetItems)]
pub async fn get_items(list_id: Uuid) -> Result<Vec<Item>, ServerFnError> {
    use crate::postgres;
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
    use crate::postgres;
    println!("get_list");
    match postgres::get_shopping_list(id).await {
        Ok(list) => {
            println!("got the list: {:?}", list);
            Ok(list)
        }
        Err(e) => {
            eprintln!("error grabbing list: {:?}", e);
            Err(ServerFnError::ServerError(
                "Could not retrieve List from Database".to_string(),
            ))
        }
    }
}

#[server(GetLists)]
pub async fn get_lists() -> Result<Vec<List>, ServerFnError> {
    use crate::postgres;
    println!("get_lists");
    match postgres::get_shopping_lists().await {
        Ok(lists) => {
            println!("got the lists: {:?}", lists);
            Ok(lists)
        }
        Err(e) => {
            eprintln!("error grabbing lists: {:?}", e);
            Err(ServerFnError::ServerError(
                "Could not retrieve Lists from Database".to_string(),
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub crossed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
