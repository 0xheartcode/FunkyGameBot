use crate::database::DbPool;
use rusqlite::{params, OptionalExtension, Error as RusqliteError};

// Function to start a new season
pub async fn start_new_season(pool: &DbPool, name: &str, max_players: i32) -> Result<(), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    conn.execute(
        "INSERT INTO seasons (name, is_active, max_players, start_date, stop_date) VALUES (?1, true, ?2, CURRENT_TIMESTAMP, NULL)",
        params![name, max_players],
    )?;
    Ok(())
}

// Function to stop the current season
pub async fn stop_current_season(pool: &DbPool) -> Result<(), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    conn.execute(
        "UPDATE seasons SET status = 'closed', is_active = false, stop_date = CURRENT_TIMESTAMP WHERE is_active = true",
        [],
    )?;
    Ok(())
}

// Function to check if a season is running and return its title
pub async fn current_active_season(pool: &DbPool) -> Result<Option<String>, RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let season_title: Option<String> = conn.query_row(
        "SELECT name FROM seasons WHERE is_active = true LIMIT 1",
        [],
        |row| row.get(0),
    ).optional()?;
    Ok(season_title)
}

// Function to check if a season is running and return its id
pub async fn current_active_season_id(pool: &DbPool) -> Result<Option<String>, RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let season_id: Option<String> = conn.query_row(
        "SELECT id FROM seasons WHERE is_active = true LIMIT 1",
        [],
        |row| row.get(0),
    ).optional()?;
    Ok(season_id)
}

// Function to get details of the current active season
pub async fn current_active_season_details(pool: &DbPool) -> Result<Option<(String, String, i32, String)>, RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let query = "
        SELECT name, start_date, max_players, status
        FROM seasons
        WHERE is_active = true
        LIMIT 1";

    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let start_date: String = row.get(1)?;
        let max_players: i32 = row.get(2)?;
        let status: String = row.get(3)?;
        Ok(Some((name, start_date, max_players, status)))
    } else {
        Ok(None)
    }
}

pub async fn start_signup_phase(pool: &DbPool) -> Result<(), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let rows_updated = conn.execute(
        "UPDATE seasons SET status = 'start_signup' WHERE is_active = TRUE AND (status = 'initial' OR status = 'stopped_signup')",
        [],
    )?;

    if rows_updated == 0 {
        Err(RusqliteError::QueryReturnedNoRows) // or use a custom error
    } else {
        Ok(())
    }
}

pub async fn stop_signup_phase(pool: &DbPool) -> Result<(), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let rows_updated = conn.execute(
        "UPDATE seasons SET status = 'stopped_signup' WHERE is_active = TRUE AND status = 'start_signup'",
        [],
    )?;

    if rows_updated == 0 {
        Err(RusqliteError::QueryReturnedNoRows) // or use a custom error
    } else {
        Ok(())
    }
}

pub async fn start_gaming_phase(pool: &DbPool) -> Result<(), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let rows_updated = conn.execute(
        "UPDATE seasons SET status = 'start_gaming' WHERE is_active = TRUE AND (status = 'stopped_signup' OR status = 'stopped_gaming')",
        [],
    )?;

    if rows_updated == 0 {
        Err(RusqliteError::QueryReturnedNoRows) // or use a custom error
    } else {
        Ok(())
    }
}

pub async fn stop_gaming_phase(pool: &DbPool) -> Result<(), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let rows_updated = conn.execute(
        "UPDATE seasons SET status = 'stopped_gaming' WHERE is_active = TRUE AND status = 'start_gaming'",
        [],
    )?;

    if rows_updated == 0 {
        Err(RusqliteError::QueryReturnedNoRows) // or use a custom error
    } else {
        Ok(())
    }
}
