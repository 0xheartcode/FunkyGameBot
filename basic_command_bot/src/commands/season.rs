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
pub async fn current_active_season_id(pool: &DbPool) -> Result<Option<i32>, RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let season_id: Option<i32> = conn.query_row(
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


// Function to get the next round number for the current active season
pub async fn get_next_round_number(pool: &DbPool, current_season_id: &i32) -> Result<i32, RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");

    // Fetch the highest round number for the current season
    let max_round_number: i32 = conn.query_row(
        "SELECT IFNULL(MAX(round_number), 0) FROM MasterRoundTable WHERE season_id = ?1",
        params![current_season_id],
        |row| row.get(0),
    )?;

    // Return 1 if no rounds exist, or the next round number if rounds do exist
    Ok(max_round_number + 1)

}


pub async fn start_new_round(pool: &DbPool, current_active_season_id_variable: i32, next_round_number_variable: i32) -> Result<(), RusqliteError> {
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // Start a transaction
    let tx = conn.transaction()?;

    // Insert a new round into MasterRoundTable
    tx.execute(
        "INSERT INTO MasterRoundTable (season_id, round_number, start_time) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
        params![current_active_season_id_variable, next_round_number_variable],
    )?;

    // Update the status in the Seasons table
    let rows_updated = tx.execute(
        "UPDATE Seasons SET status = 'round_ongoing' WHERE id = ?1 AND status = 'start_gaming'",
        params![current_active_season_id_variable],
    )?;

    if rows_updated == 0 {
        // No rows were updated
        Err(RusqliteError::QueryReturnedNoRows) // or a custom error
    } else {
        // Commit the transaction
        tx.commit()?;
        Ok(())
    }    
}


pub async fn end_current_round(pool: &DbPool, current_active_season_id_variable: i32) -> Result<(), RusqliteError> {
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // Start a transaction
    let tx = conn.transaction()?;

    // Insert a new round into MasterRoundTable
    tx.execute(
        "UPDATE MasterRoundTable SET end_time = CURRENT_TIMESTAMP WHERE season_id = ?1 AND end_time IS NULL",
        params![current_active_season_id_variable],
    )?;

    // Update the status in the Seasons table
    let rows_updated = tx.execute(
        "UPDATE Seasons SET status = 'start_gaming' WHERE id = ?1 AND status = 'round_ongoing'",
        params![current_active_season_id_variable],
    )?;

    if rows_updated == 0 {
        // No rows were updated
        Err(RusqliteError::QueryReturnedNoRows) // or a custom error
    } else {
        // Commit the transaction
        tx.commit()?;
        Ok(())
    }
        
}
