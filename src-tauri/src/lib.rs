use tauri::Manager;

use crate::db::DbState;

mod commands;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            tauri::async_runtime::block_on(async move {
                let pool = db::create_db_pool(&handle)
                    .await
                    .expect("failed to initialize db pool");

                handle.manage(DbState { pool });
            });

            Ok(())
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_steam_owned_games,
            commands::get_game_info,
            commands::is_game_installed,
            commands::launch_game,
            commands::install_game,
            commands::filter_games
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
