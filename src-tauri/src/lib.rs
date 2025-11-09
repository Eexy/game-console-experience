use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct SteamOwnedGame {
    appid: u32,
    name: String,
    playtime_2weeks: Option<u32>,
    playtime_forever: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SteamGamesData {
    game_count: u32,
    games: Vec<SteamOwnedGame>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SteamApiResponse {
    response: SteamGamesData,
}

#[tauri::command]
async fn get_steam_owned_games(
    profile_id: String,
    steam_key: String,
) -> Result<SteamGamesData, String> {
    let url = format!(
        "http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&include_appinfo=1&format=json",
        steam_key, profile_id
    );

    println!("{}", url);

    let res = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    let body = res.text().await.map_err(|e| e.to_string())?;

    println!("{}", body);

    let api_response: SteamApiResponse = serde_json::from_str(&body).map_err(|e| e.to_string())?;

    Ok(api_response.response)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_steam_owned_games])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
