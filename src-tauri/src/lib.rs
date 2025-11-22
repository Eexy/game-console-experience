use tauri::Manager;

use crate::{commands::SteamState, db::DbState};

mod commands;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            tauri::async_runtime::block_on(async {
                let pool = db::create_db_pool(handle)
                    .await
                    .expect("failed to initialize db pool");

                app.manage(DbState { pool });
            });

            app.manage(SteamState::new());

            Ok(())
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_games,
            commands::get_game_by_id,
            commands::filter_games_by_title,
            commands::refresh_games
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
