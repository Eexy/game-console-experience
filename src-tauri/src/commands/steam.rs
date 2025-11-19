use std::path::PathBuf;
use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use tauri_plugin_http::reqwest;
use tauri_plugin_opener::OpenerExt;
use windows_registry::LOCAL_MACHINE;

#[derive(Debug)]
pub struct SteamState {
    pub steam_key: String,
    pub profile_id: String,
}

impl SteamState {
    pub fn new() -> Self {
        let cwd = std::env::current_dir()
            .expect("unable to get current working directory")
            .join(".env");

        let content = fs::read_to_string(cwd).expect("unable to read env file");

        let mut profile_id = None;
        let mut steam_key = None;

        for line in content.lines() {
            if let Some((key, value)) = line.split_once("=") {
                match key.trim() {
                    "STEAM_KEY" => steam_key = Some(value.trim().to_string()),
                    "PROFILE_ID" => profile_id = Some(value.trim().to_string()),
                    _ => {}
                }
            }
        }

        SteamState {
            steam_key: steam_key.expect("unable to read steam_key from env"),
            profile_id: profile_id.expect("unable to read profile_id from env"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamOwnedGame {
    appid: u32,
    name: String,
    playtime_2weeks: Option<u32>,
    playtime_forever: u32,
    description: Option<String>,
    img_icon_url: Option<String>,
    img_hero: Option<String>,
    img_logo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamGamesData {
    games: Vec<SteamOwnedGame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamApiResponse {
    response: SteamGamesData,
}

pub async fn get_steam_games(
    pool: &Pool<Sqlite>,
    profile_id: String,
    steam_key: String,
) -> Result<(), String> {
    let url = format!(
        "http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&include_appinfo=1&format=json",
        steam_key, profile_id
    );

    let res = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    let body = res.text().await.map_err(|e| e.to_string())?;

    let api_response: SteamApiResponse = serde_json::from_str(&body).map_err(|e| e.to_string())?;

    if api_response.response.games.is_empty() {
        return Ok(());
    }

    let placeholders = api_response
        .response
        .games
        .iter()
        .map(|_| "(?, ?, ? )")
        .collect::<Vec<_>>()
        .join(", ");

    let query_str = format!(
        "insert into games (title, logo_url, store_app_id) values {}",
        placeholders
    );

    let mut query = sqlx::query(&query_str);

    for game in &api_response.response.games {
        query = query
            .bind(&game.name)
            .bind(&game.img_icon_url)
            .bind(game.appid);
    }

    query.execute(pool).await.map_err(|e| e.to_string())?;

    Ok(())
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
    pub name: String,
    pub steam_appid: u32,
    pub required_age: RequiredAge,
    pub is_free: bool,
    pub about_the_game: String,
    pub header_image: String,
    pub capsule_image: String,
    pub genres: Vec<SteamGameGenre>,
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

pub async fn get_game_info(game_id: u32) -> Result<SteamGameInfo, String> {
    let url = format!(
        "https://store.steampowered.com/api/appdetails?appids={}",
        game_id
    );

    let res = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    let body = res.text().await.map_err(|e| e.to_string())?;

    let mut api_response: SteamGameInfoResponse =
        serde_json::from_str(&body).map_err(|e| e.to_string())?;

    match api_response.games.remove(&game_id.to_string()) {
        Some(info) => {
            if info.success {
                Ok(info.data)
            } else {
                Err("unable to get game info".to_string())
            }
        }
        None => Err("unable to get game info".to_string()),
    }
}

#[tauri::command]
pub async fn is_game_installed(game_id: u32) -> bool {
    let mut raw_steam_path = get_steam_path().expect("unable to get steam path");
    raw_steam_path.push("steamapps");
    raw_steam_path.push(format!("appmanifest_{}.acf", game_id));

    let steam_path = raw_steam_path.to_str().expect("unable to get steam path");

    let path = Path::new(&steam_path);

    path.exists()
}

#[tauri::command]
pub async fn launch_game(app: tauri::AppHandle, game_id: u32) -> Result<bool, String> {
    let steam_url = format!("steam://install/{}/", game_id,);

    let res = app.opener().open_url(steam_url, None::<&str>);

    match res {
        Ok(_) => Ok(true),
        Err(_) => Err("unable to launch_game".to_string()),
    }
}

#[tauri::command]
pub async fn install_game(app: tauri::AppHandle, game_id: u32) -> Result<bool, String> {
    let steam_url = format!("steam://run/{}/", game_id,);

    let res = app.opener().open_url(steam_url, None::<&str>);

    match res {
        Ok(_) => Ok(true),
        Err(_) => Err("unable to launch_game".to_string()),
    }
}

#[tauri::command]
pub fn filter_games(games: Vec<SteamOwnedGame>, search: String) -> Vec<SteamOwnedGame> {
    let filtered: Vec<SteamOwnedGame> = games
        .into_iter()
        .filter(|game| game.name.to_lowercase().contains(&search.to_lowercase()))
        .collect();

    sort_game(filtered)
}

fn sort_game(mut games: Vec<SteamOwnedGame>) -> Vec<SteamOwnedGame> {
    games.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    games
}

fn get_steam_path() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        get_steam_path_windows()
    }

    #[cfg(not(windows))]
    {
        None
    }
}

#[cfg(target_os = "windows")]
fn get_steam_path_windows() -> Option<PathBuf> {
    //  LOCAL_MACHINE
    if let Ok(install_path) = LOCAL_MACHINE
        .open(r"SOFTWARE\WOW6432Node\Valve\Steam")
        .and_then(|key| key.get_string("InstallPath"))
    {
        return Some(PathBuf::from(install_path));
    }

    //  CURRENT_USER
    if let Ok(install_path) = LOCAL_MACHINE
        .open(r"Software\Valve\Steam")
        .and_then(|key| key.get_string("SteamPath"))
    {
        return Some(PathBuf::from(install_path));
    }

    None
}
