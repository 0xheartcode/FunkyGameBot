/// registration_commands.rs

//use teloxide::{prelude::* };
//use std::{error::Error, sync::Arc};
use crate::database::{DbPool};
use rusqlite::{params, OptionalExtension, Error as RusqliteError};

//use crate::admin::{is_authorized_sender };

use crate::commands::season:: {
    current_active_season_details
};



pub async fn insert_player_hand_choice(db_pool: &DbPool, round_id: i32, player_id: i64, player_hand: &str) -> Result<bool, RusqliteError> {
    let conn = db_pool.get().expect("Failed to get DB connection");

    // Check if the player has already played in this round
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM RoundDetailsTable WHERE round_id = ?1 AND player_id = ?2",
        params![round_id, player_id],
        |row| row.get(0),
    ).unwrap_or(0);

    // If the player has already played, return false
    if exists > 0 {
        return Ok(false);
    }

    // Insert the player's hand choice
    let rows_affected = conn.execute(
        "INSERT INTO RoundDetailsTable (round_id, player_id, player_hand, timestamp) VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)",
        params![round_id, player_id, player_hand],
    )?;

    // Return true if the row was successfully inserted
    Ok(rows_affected > 0)
}

// Retrieve the current game status and season_id
pub async fn current_game_status_and_season_id(db_pool: &DbPool) -> Result<Option<(String, i32)>, RusqliteError> {
    let season_details = current_active_season_details(db_pool).await?;
    Ok(season_details.map(|(season_id, _,  _, _, status)| (status, season_id)))
}

// Check if the player is in the current game
pub async fn check_player_in_game(db_pool: &DbPool, player_id: i64, season_id: i32) -> Result<bool, RusqliteError> {
    let conn = db_pool.get().expect("Failed to get DB connection");
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM PlayerDetailsTable WHERE player_id = ?1 AND season_id = ?2",
        params![player_id, season_id],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

// Get the current round ID
pub async fn get_current_round_id(db_pool: &DbPool, season_id: i32) -> Result<Option<i32>, RusqliteError> {
    let conn = db_pool.get().expect("Failed to get DB connection");
    let round_id: Option<i32> = conn.query_row(
        "SELECT id FROM MasterRoundTable WHERE season_id = ?1 AND end_time IS NULL ORDER BY id DESC LIMIT 1",
        params![season_id],
        |row| row.get(0),
    ).optional()?;
    Ok(round_id)
}
