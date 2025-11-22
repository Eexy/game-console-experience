use serde::Serialize;
use sqlx::{Pool, Sqlite};
use tauri::State;

use crate::db::DbState;
use crate::steam::{get_game_info, get_steam_games, SteamState};

#[derive(sqlx::FromRow, Serialize)]
pub struct Game {
    id: u16,
    title: String,
    store_app_id: u32,
    logo_url: Option<String>,
    description: Option<String>,
}

/// Get games from local db
#[tauri::command]
pub async fn get_games(db_state: State<'_, DbState>) -> Result<Vec<Game>, String> {
    let games = sqlx::query_as::<_, Game>(
        "
         select id, title, logo_url, store_app_id, description
         from games
         order by title
        ",
    )
    .fetch_all(&db_state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(games)
}

/// Filter games by title
#[tauri::command]
pub async fn filter_games_by_title(
    db_state: State<'_, DbState>,
    title: String,
) -> Result<Vec<Game>, String> {
    let search_pattern = title.to_lowercase();

    let games = sqlx::query_as::<_, Game>(
        "
         select id, title, logo_url, store_app_id, description
         from games
         where lower(title) like '%' || ? || '%'
         order by title
        ",
    )
    .bind(search_pattern)
    .fetch_all(&db_state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(games)
}

/// Refresh games list by deleting all saved games in db, get games from steam and then info from
/// IGDB
#[tauri::command]
pub async fn refresh_games(
    state: State<'_, DbState>,
    steam_state: State<'_, SteamState>,
) -> Result<(), String> {
    delete_games(&state.pool).await.map_err(|e| e.to_string())?;

    get_steam_games(
        &state.pool,
        steam_state.profile_id.clone(),
        steam_state.steam_key.clone(),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Get game by id with extra info like description, artwork...
#[tauri::command]
pub async fn get_game_by_id(state: State<'_, DbState>, id: u32) -> Result<Game, String> {
    let game = sqlx::query_as::<_, Game>(
        "
         select id, title, logo_url, store_app_id, description
         from games
         where id = ?
         order by title
        ",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    if game.description.is_some() {
        Ok(game)
    } else {
        let infos = get_game_info(game.store_app_id)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(
            "
        update games
        set description = ?, img_hero = ?
        where id = ?
     
",
        )
        .bind(infos.about_the_game)
        .bind(infos.header_image)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

        let updated_games = sqlx::query_as::<_, Game>(
            "
         select id, title, logo_url, store_app_id, description
         from games
         where id = ?
         order by title
        ",
        )
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(updated_games)
    }
}

/// Delete all games from db
async fn delete_games(pool: &Pool<Sqlite>) -> Result<(), String> {
    sqlx::query("delete from games")
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
