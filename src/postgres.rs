use crate::common_types::*;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use log::info;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use sqlx::{
    migrate::MigrateError,
    postgres::{PgPoolOptions, PgRow, Postgres},
    FromRow, Pool, Row,
};

#[cfg(feature = "ssr")]
pub async fn get_pg_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    // Don't create a pool for every request.
    // Figure out how to do it once and pass it around as state
    let username = "postgres";
    let password = "password";
    let hostname = "localhost";
    let port = 5432;
    let dbname = "postgres";
    let connect_string = format!("postgres://{username}:{password}@{hostname}:{port}/{dbname}");
    info!("connecting to {connect_string}");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&connect_string)
        .await
}

#[cfg(feature = "ssr")]
pub async fn run_db_migrations() -> Result<(), MigrateError> {
    let pool = get_pg_pool().await?;
    sqlx::migrate!("./migrations").run(&pool).await
}

#[cfg(feature = "ssr")]
pub async fn get_shopping_lists() -> Result<Vec<List>, sqlx::Error> {
    let pool = get_pg_pool().await?;
    let rows: Vec<PgRow> = sqlx::query("SELECT * FROM shopping_lists")
        .fetch_all(&pool)
        .await?;
    rows.iter().map(|r| List::from_row(r)).collect()
}

#[cfg(feature = "ssr")]
pub async fn create_shopping_list(name: &str) -> Result<List, sqlx::Error> {
    let pool = get_pg_pool().await?;

    let row: PgRow = sqlx::query(
        r#"
            INSERT INTO shopping_lists ( name )
            VALUES ( $1 )
            RETURNING *
        "#,
    )
    .bind(name)
    .fetch_one(&pool)
    .await?;
    List::from_row(&row)
}

////////////////////

#[cfg(feature = "ssr")]
pub async fn get_list_items(list_id: Uuid) -> Result<Vec<Item>, sqlx::Error> {
    let pool = get_pg_pool().await?;
    let rows: Vec<PgRow> = sqlx::query("SELECT * FROM items WHERE parent_id = $1")
        .bind(list_id)
        .fetch_all(&pool)
        .await?;
    rows.iter().map(|r| Item::from_row(r)).collect()
}

#[cfg(feature = "ssr")]
pub async fn create_list_item(name: &str, list_id: Uuid) -> Result<Item, sqlx::Error> {
    let pool = get_pg_pool().await?;

    let row: PgRow = sqlx::query(
        r#"
            INSERT INTO items ( name, parent_id, crossed )
            VALUES ( $1, $2, false )
            RETURNING *
        "#,
    )
    .bind(name)
    .bind(list_id)
    .fetch_one(&pool)
    .await?;
    Item::from_row(&row)
}
