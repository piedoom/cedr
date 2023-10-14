use shared::{models, InputMethod};
use sqlx::{Pool, Sqlite};
use tauri::State;

use crate::{api::*, cedict};

#[tauri::command]
pub(crate) async fn query(
    pool: State<'_, Pool<Sqlite>>,
    method: InputMethod,
    query: String,
    include_sentences: bool,
) -> anyhow::Result<Vec<models::Entry>, ()> {
    entries::query(pool.inner(), query, method).await
}

#[tauri::command]
pub async fn history_index(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<models::Entry>, ()> {
    Ok(history::index(pool.inner()).await.unwrap())
}

#[tauri::command]
pub async fn history_create(pool: State<'_, Pool<Sqlite>>, term_id: i64) -> Result<(), ()> {
    history::create(pool.inner(), term_id).await.unwrap();
    Ok(())
}

/// Initialize cedict into memory, and download the dictionary if it doesn't exist
#[tauri::command]
pub(crate) async fn initialize_dictionary_command(
    pool: State<'_, Pool<Sqlite>>,
) -> std::result::Result<(), ()> {
    cedict::build_dictionary(pool.inner()).await.unwrap();
    Ok(())
}

#[tauri::command]
pub async fn get_term(
    pool: State<'_, Pool<Sqlite>>,
    id: u32,
    include_sentences: bool,
) -> Result<models::Entry, ()> {
    Ok(entries::get(pool.inner(), id).await.unwrap())
}

#[tauri::command]
pub(crate) async fn collections_index(
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<models::Collection>, ()> {
    Ok(collections::index(pool.inner()).await.unwrap())
}

#[tauri::command]
pub(crate) async fn collections_get(
    pool: State<'_, Pool<Sqlite>>,
    id: u32,
) -> Result<models::CollectionWithEntries, ()> {
    Ok(collections::get(pool.inner(), id).await.unwrap())
}

#[tauri::command]
pub(crate) async fn collections_create(
    pool: State<'_, Pool<Sqlite>>,
    name: String,
) -> Result<u32, ()> {
    Ok(collections::create(pool.inner(), name).await.unwrap())
}

#[tauri::command]
pub(crate) async fn collections_add_term(
    pool: State<'_, Pool<Sqlite>>,
    collection_id: u32,
    term_id: u32,
) -> Result<(), ()> {
    collections::add_term(pool.inner(), collection_id, term_id)
        .await
        .unwrap();
    Ok(())
}
