use serde::Serialize;
use tauri::State;

use crate::{commands::get_steam_games, db::DbState};

#[derive(sqlx::FromRow, Serialize)]
pub struct Game {
    id: u16,
    title: String,
    logo_url: Option<String>,
}

#[tauri::command]
pub async fn get_games(state: State<'_, DbState>) -> Result<Vec<Game>, String> {
    let games = sqlx::query_as::<_, Game>(
        "
 select id, title, logo_url
 from games
",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(games)
}

#[tauri::command]
pub async fn refresh_games(
    state: State<'_, DbState>,
    profile_id: String,
    steam_key: String,
) -> Result<(), String> {
    get_steam_games(&state.pool, profile_id, steam_key)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
