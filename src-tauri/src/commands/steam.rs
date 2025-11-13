use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamOwnedGame {
    appid: u32,
    name: String,
    playtime_2weeks: Option<u32>,
    playtime_forever: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamGamesData {
    game_count: u32,
    games: Vec<SteamOwnedGame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamApiResponse {
    response: SteamGamesData,
}

#[tauri::command]
pub async fn get_steam_owned_games(
    profile_id: String,
    steam_key: String,
) -> Result<SteamGamesData, String> {
    let url = format!(
        "http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&include_appinfo=1&format=json",
        steam_key, profile_id
    );

    let res = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    let body = res.text().await.map_err(|e| e.to_string())?;

    let api_response: SteamApiResponse = serde_json::from_str(&body).map_err(|e| e.to_string())?;

    Ok(api_response.response)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamGameGenre {
    id: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequiredAge {
    Number(u16),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamGameInfo {
    name: String,
    steam_appid: u32,
    required_age: RequiredAge,
    is_free: bool,
    about_the_game: String,
    header_image: String,
    capsule_image: String,
    genres: Vec<SteamGameGenre>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfoData {
    success: bool,
    data: SteamGameInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamGameInfoResponse {
    #[serde(flatten)]
    pub games: HashMap<String, GameInfoData>,
}

#[tauri::command]
pub async fn get_game_info(game_id: u32) -> Result<HashMap<String, GameInfoData>, String> {
    let url = format!(
        "https://store.steampowered.com/api/appdetails?appids={}",
        game_id
    );

    let res = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    let body = res.text().await.map_err(|e| e.to_string())?;

    println!("{}", &body);

    let api_response: SteamGameInfoResponse =
        serde_json::from_str(&body).map_err(|e| e.to_string())?;

    Ok(api_response.games)
}
