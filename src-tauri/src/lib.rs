mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_steam_owned_games,
            commands::get_game_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
