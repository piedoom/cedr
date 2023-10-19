#![feature(core_intrinsics)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod api;
mod cedict;
mod commands;

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env, str::FromStr};
use strum_macros::{Display, EnumString};
use tauri::menu::{MenuBuilder, MenuId, MenuItem, SubmenuBuilder};

#[derive(Debug, EnumString, Display, Default)]
enum MenuAction {
    #[default]
    None,
    UpdateDictionary,
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

    dbg!(&database_path);

    if !sqlx::Sqlite::database_exists(database_path).await.unwrap() {
        sqlx::Sqlite::create_database(database_path).await.unwrap();
    }

    // let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let pool = SqlitePoolOptions::new()
        .connect(database_path)
        .await
        .unwrap();

    sqlx::migrate!("../migrations/").run(&pool).await.unwrap();

    let manage_pool = pool.clone();

    tauri::Builder::default()
        .menu(|app| {
            MenuBuilder::new(app)
                .item(
                    &SubmenuBuilder::new(app, "Settings")
                        .items(&[
                            // &MenuItem::new(app, "Settings", true, None),
                            &MenuItem::with_id(
                                app,
                                &MenuAction::UpdateDictionary,
                                "Update dictionary",
                                true,
                                None,
                            ),
                        ])
                        .build()?,
                )
                // .item(
                //     &SubmenuBuilder::new(app, "Edit")
                //         .items(&[
                //             &MenuItem::new(app, "Edit collections", true, None),
                //             &MenuItem::new(app, "Update dictionary", true, None),
                //         ])
                //         .build()?,
                // )
                .build()
        })
        .setup(move |app| {
            let pool = pool.clone();
            app.on_menu_event(move |_app, event| {
                let pool = pool.clone();
                match MenuAction::from(event.id) {
                    MenuAction::UpdateDictionary => {
                        tokio::spawn(async move {
                            cedict::build_dictionary(&pool).await.unwrap();
                        });
                        // std::thread::spawn(move || block_on(cedict::build_dictionary(&pool.clone())));
                        // app.trigger_global(MenuAction::UpdateDictionary.to_string().as_str(), None)
                        // cedict::build_dictionary(&pool).await.unwrap();
                    }
                    MenuAction::None => (),
                }
            });
            Ok(())
        })
        .manage(manage_pool.clone())
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
