use shared::{models, InputMethod, Int};
use sqlx::{Pool, Sqlite};
use tauri::State;
use tauri_plugin_dialog::DialogExt;

use crate::{
    api::{self, *},
    cedict,
};

#[inline(always)]
fn tauri_err(e: impl ToString) -> String {
    e.to_string()
}

#[tauri::command]
pub(crate) async fn query(
    pool: State<'_, Pool<Sqlite>>,
    method: InputMethod,
    query: String,
) -> anyhow::Result<Vec<models::Entry>, String> {
    entries::query(pool.inner(), &query, &method)
        .await
        .map_err(tauri_err)
}

#[tauri::command]
pub async fn history_index(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<models::Entry>, String> {
    history::index(pool.inner()).await.map_err(tauri_err)
}

#[tauri::command]
pub async fn history_create(pool: State<'_, Pool<Sqlite>>, term_id: Int) -> Result<Int, String> {
    history::create(pool.inner(), term_id)
        .await
        .map_err(tauri_err)
}

/// Initialize cedict into memory, and download the dictionary if it doesn't exist
#[tauri::command]
pub(crate) async fn initialize_dictionary_command(
    pool: State<'_, Pool<Sqlite>>,
) -> std::result::Result<(), String> {
    cedict::build_dictionary(pool.inner())
        .await
        .map_err(tauri_err)
}

#[tauri::command]
pub async fn get_term(pool: State<'_, Pool<Sqlite>>, id: Int) -> Result<models::Entry, String> {
    entries::get(pool.inner(), id).await.map_err(tauri_err)
}

#[tauri::command]
pub async fn entry_get_by_traditional(
    pool: State<'_, Pool<Sqlite>>,
    traditional: String,
) -> Result<models::Entry, String> {
    entries::get_by_traditional(pool.inner(), traditional)
        .await
        .map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn collections_index(
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<models::Collection>, String> {
    collections::index(pool.inner()).await.map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn collections_get(
    pool: State<'_, Pool<Sqlite>>,
    id: Int,
) -> Result<models::CollectionWithEntries, String> {
    collections::get(pool.inner(), id).await.map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn collections_create(
    pool: State<'_, Pool<Sqlite>>,
    name: String,
) -> Result<Int, String> {
    collections::create(pool.inner(), name)
        .await
        .map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn collections_add_term(
    pool: State<'_, Pool<Sqlite>>,
    collection_id: Int,
    term_id: Int,
) -> Result<Int, String> {
    collections::add_term(pool.inner(), collection_id, term_id)
        .await
        .map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn scores_get_or_create(
    pool: State<'_, Pool<Sqlite>>,
    entry_id: Int,
) -> Result<models::Score, String> {
    scores::get_or_create(pool.inner(), entry_id)
        .await
        .map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn scores_get(
    pool: State<'_, Pool<Sqlite>>,
    entry_id: Int,
) -> Result<models::Score, String> {
    scores::get(pool.inner(), entry_id).await.map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn scores_update(
    pool: State<'_, Pool<Sqlite>>,
    entry_id: Int,
    correct: bool,
) -> Result<models::Score, String> {
    scores::update(pool.inner(), entry_id, correct)
        .await
        .map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn update_cedict(pool: State<'_, Pool<Sqlite>>) -> Result<(), String> {
    cedict::build_dictionary(pool.inner())
        .await
        .map_err(tauri_err)?;
    Ok(())
}

#[tauri::command]
pub(crate) async fn import_collections(
    app_handle: tauri::AppHandle,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    let pool = pool.inner().clone();
    let file = app_handle.dialog().file().blocking_pick_file().unwrap();
    api::collections::import(&pool, file.path)
        .await
        .map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn export_collections(
    app_handle: tauri::AppHandle,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    let pool = pool.inner().clone();
    let file = app_handle.dialog().file().blocking_pick_file().unwrap();
    api::collections::export(&pool, file.path)
        .await
        .map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn get_preferences() -> Result<models::Preferences, String> {
    api::preferences::get_or_create().await.map_err(tauri_err)
}

#[tauri::command]
pub(crate) async fn set_preferences(theme: models::Theme) -> Result<(), String> {
    let preferences = models::Preferences { theme };
    api::preferences::save(preferences).await.map_err(tauri_err)
}
