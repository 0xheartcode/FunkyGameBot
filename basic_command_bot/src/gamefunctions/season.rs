use crate::database::DbPool;
use rusqlite::{params, OptionalExtension, Error as RusqliteError};

// Function to start a new season
pub async fn start_new_season(pool: &DbPool, name: &str, max_players: i32) -> Result<(), RusqliteError> {
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO seasons (name, is_active, max_players, start_date, stop_date) VALUES (?1, true, ?2, CURRENT_TIMESTAMP, NULL)",
        params![name, max_players],
    )?;
    Ok(())
}

// Function to stop the current season
pub async fn stop_current_season(pool: &DbPool) -> Result<(), RusqliteError> {
    let conn = pool.get()?;
    conn.execute(
        "UPDATE seasons SET is_active = false, stop_date = CURRENT_TIMESTAMP WHERE is_active = true",
        [],
    )?;
    Ok(())
}

// Function to check if a season is running and return its title
pub async fn current_active_season(pool: &DbPool) -> Result<Option<String>, RusqliteError> {
    let conn = pool.get()?;
    let season_title: Option<String> = conn.query_row(
        "SELECT name FROM seasons WHERE is_active = true LIMIT 1",
        [],
        |row| row.get(0),
    ).optional()?;
    Ok(season_title)
}

