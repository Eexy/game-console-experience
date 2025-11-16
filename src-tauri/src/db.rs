use sqlx::{sqlite::SqliteConnectOptions, sqlite::SqlitePool};
use tauri::Manager;

pub async fn create_db_pool(app_handle: &tauri::AppHandle) -> Result<SqlitePool, sqlx::Error> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data directory");

    std::fs::create_dir_all(&app_data_dir).expect("failed to create app data directory");

    let db_path = app_data_dir.join("rocade.db");

    dbg!(&db_path);

    let connection_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = SqlitePool::connect_with(connection_options).await?;

    Ok(pool)
}

pub struct DbState {
    pub pool: SqlitePool,
}
