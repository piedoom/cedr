#![feature(core_intrinsics)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod api;
mod cedict;
mod commands;

use std::env;

use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = dotenv::var("DATABASE_URL").unwrap();

    if !sqlx::Sqlite::database_exists(&url).await.unwrap() {
        sqlx::Sqlite::create_database(&url).await.unwrap();
    }

    // let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // let migrations = Path::new(&crate_dir).join("./migrations");

    let pool = SqlitePoolOptions::new()
        .connect(url.as_str())
        .await
        .unwrap();

    // sqlx::migrate::Migrator::new(migrations)
    //     .await
    //     .unwrap()
    //     .run(&pool)
    //     .await
    //     .unwrap();

    // TODO: only update conditionally. This attempts to rebuild tables every start
    cedict::build_dictionary(&pool).await.unwrap();

    tauri::Builder::default()
        .manage(pool.clone())
        .invoke_handler(tauri::generate_handler![
            commands::query,
            commands::history_index,
            commands::history_create,
            commands::initialize_dictionary_command,
            commands::get_term,
            commands::collections_index,
            commands::collections_get,
            commands::collections_create,
            commands::collections_add_term,
            // TODO: Get term
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
