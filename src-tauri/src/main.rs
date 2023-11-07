#![feature(core_intrinsics)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod api;
mod cedict;
mod commands;

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use log::LevelFilter;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env, str::FromStr};
use strum_macros::{Display, EnumString};
use tauri::menu::MenuId;
use tauri_plugin_log::{Target, TargetKind};

#[derive(Debug, EnumString, Display, Default)]
enum MenuAction {
    #[default]
    None,
    UpdateDictionary,
    ExportCollections,
    ImportCollections,
}

impl From<MenuId> for MenuAction {
    fn from(value: MenuId) -> Self {
        MenuAction::from_str(&value.0).unwrap_or_default()
    }
}

#[tokio::main]
async fn main() {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "doomy".to_string(),
        author: "doomy".to_string(),
        app_name: "cedr".to_string(),
    })
    .unwrap();

    let data_path = strategy.data_dir();

    fs_extra::dir::create_all(&data_path, false).unwrap();

    let database_path = data_path.join("data.sqlite");
    let database_path = database_path.to_str().unwrap();

    if !sqlx::Sqlite::database_exists(database_path).await.unwrap() {
        sqlx::Sqlite::create_database(database_path).await.unwrap();
    }

    let pool = SqlitePoolOptions::new()
        .connect(database_path)
        .await
        .unwrap();

    if let Err(e) = sqlx::migrate!("../migrations/").run(&pool).await {
        println!("{e}");
    }

    let manage_pool = pool.clone();

    tauri::Builder::default()
        .manage(manage_pool.clone())
        .invoke_handler(tauri::generate_handler![
            commands::query,
            commands::history_index,
            commands::history_create,
            commands::initialize_dictionary_command,
            commands::get_term,
            commands::entry_get_by_traditional,
            commands::collections_index,
            commands::collections_get,
            commands::collections_create,
            commands::collections_add_term,
            commands::scores_get_or_create,
            commands::scores_get,
            commands::scores_update,
            commands::update_cedict,
            commands::import_collections,
            commands::export_collections,
            commands::get_preferences,
            commands::set_preferences,
        ])
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(TargetKind::LogDir {
                        file_name: Some(data_path.join("log").to_string_lossy().to_string()),
                    }),
                    Target::new(TargetKind::Stdout),
                ])
                .level(LevelFilter::Info)
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
