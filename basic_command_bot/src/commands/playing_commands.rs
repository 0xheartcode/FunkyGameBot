/// playing_commands.rs

//use teloxide::{prelude::* };
//use std::{error::Error, sync::Arc};
use crate::database::{DbPool};
use rusqlite::{params, OptionalExtension, Error as RusqliteError};

//use crate::admin::{is_authorized_sender };

use crate::commands::season:: {
    current_active_season_details
};
use rand::seq::SliceRandom;
use rand::Rng;

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


pub async fn get_player_hands(db_pool: &DbPool, round_id: i32, season_id: i32) -> Result<Vec<(i64, String)>, RusqliteError> {
    let conn = db_pool.get().expect("Failed to get DB connection");
    let mut stmt = conn.prepare(
        "SELECT rdt.player_id, rdt.player_hand
         FROM RoundDetailsTable rdt
         JOIN MasterRoundTable mrt ON rdt.round_id = mrt.id
         WHERE rdt.round_id = ?1 AND mrt.season_id = ?2",
    )?;
    let player_hands = stmt.query_map(params![round_id, season_id], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?
    .collect::<Result<Vec<(i64, String)>, RusqliteError>>()?;

    Ok(player_hands)
}

pub async fn random_match_players(player_hands: Vec<(i64, String)>) -> Vec<((i64, String), (i64, String))> {
    let mut rng = rand::thread_rng();
    let mut shuffled_hands = player_hands.clone();
    shuffled_hands.shuffle(&mut rng);

    let mut matched_pairs = Vec::new();

    if shuffled_hands.len() % 2 != 0 {
        println!("Uneven number of players triggered");
        // If there's an odd number of players, select a random player to play twice
        let random_index = rng.gen_range(0..shuffled_hands.len());
        let player_to_play_twice = shuffled_hands.remove(random_index);
        matched_pairs.push((player_to_play_twice.clone(), player_to_play_twice.clone()));
    }

    while let Some(player_one) = shuffled_hands.pop() {
        if let Some(player_two) = shuffled_hands.pop() {
            matched_pairs.push((player_one, player_two));
        }
    }

    matched_pairs
}

pub async fn evaluate_matches(
    _db_pool: &DbPool,
    matches: Vec<((i64, String), (i64, String))>,
    round_id: i32
) -> Result<Vec<(i64, String, i64, String, String, i32)>, RusqliteError> {
    let mut results = Vec::new();

    for ((player_id, player_hand), (opponent_id, opponent_hand)) in matches {
        let game_status = match (player_hand.as_str(), opponent_hand.as_str()) {
            // Both players have empty hands
            ("", "") => "draw",
            // Player wins when opponent has an empty hand and player doesn't
            (_, "") => "won",
            // Player loses when their hand is empty and opponent's isn't
            ("", _) => "lost",
            // Existing logic for non-empty hands
            ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => "won",
            ("scissors", "rock") | ("paper", "scissors") | ("rock", "paper") => "lost",
            // Draw when both hands are the same (excluding both empty, handled above)
            _ => "draw",
        };

        results.push((player_id, player_hand, opponent_id, opponent_hand, game_status.to_string(), round_id));
    }

    Ok(results)
}

pub async fn update_player_score(
    db_pool: &DbPool,
    match_results: Vec<(i64, String, i64, String, String, i32)>,
    season_id: i32
) -> Result<(), RusqliteError> {
    let mut conn = db_pool.get().expect("Failed to get DB connection");
    let tx = conn.transaction()?;

    for (player_id, _player_hand, opponent_id, opponent_hand, game_status, round_id) in match_results {
        // Fetch player username
        let player_conn = db_pool.get().expect("Failed to get DB connection");
        let player_username: String = player_conn.query_row(
            "SELECT player_username FROM PlayerDetailsTable WHERE player_id = ?1 AND season_id = ?2",
            params![player_id, season_id],
            |row| row.get(0),
        )?;

        // Use a separate connection to fetch opponent username
        let opponent_conn = db_pool.get().expect("Failed to get DB connection");
        let opponent_username: String = opponent_conn.query_row(
            "SELECT player_username FROM PlayerDetailsTable WHERE player_id = ?1 AND season_id = ?2",
            params![opponent_id, season_id],
            |row| row.get(0),
        )?;


        let score_increment = match game_status.as_str() {
            "won" => 2,
            "draw" => 1,
            _ => 0,
        };

        // Update PlayerDetailsTable for the current season for the first player
        tx.execute(
            "UPDATE PlayerDetailsTable SET score = score + ? WHERE player_id = ? AND season_id = ?",
            params![score_increment, player_id, season_id],
        )?;

        // Update RoundDetailsTable for the current round for the first player
        tx.execute(
            "UPDATE RoundDetailsTable SET opponent = ?, opponent_hand = ?, game_status = ?, player_username = ?, opponent_username = ? WHERE player_id = ? AND round_id = ?",
            params![opponent_id, opponent_hand, game_status, player_username, opponent_username, player_id, round_id],
        )?;

        // Determine score increment for the opponent
        let opponent_score_increment = match game_status.as_str() {
            "lost" => 2,  // Opponent won
            "draw" => 1,  // Draw
            _ => 0,       // Opponent lost
        };

        // Update PlayerDetailsTable for the opponent
        tx.execute(
            "UPDATE PlayerDetailsTable SET score = score + ? WHERE player_id = ? AND season_id = ?",
            params![opponent_score_increment, opponent_id, season_id],
        )?;

        // Update RoundDetailsTable for the opponent
        tx.execute(
            "UPDATE RoundDetailsTable SET opponent = ?, opponent_hand = ?, opponent_username = ?, player_username = ?, game_status = ? WHERE player_id = ? AND round_id = ?",
            params![
                player_id, 
                _player_hand,
                player_username,
                opponent_username,
                match game_status.as_str() {
                    "won" => "lost",
                    "lost" => "won",
                    _ => "draw",
                }, 
                opponent_id,
                round_id
            ],
        )?;


    }

    tx.commit()?;
    Ok(())
}

pub async fn announce_results(
    db_pool: &DbPool,
    match_results: Vec<(i64, String, i64, String, String, i32)>,
) -> Result<String, RusqliteError> {
    let mut announcement = String::new();

    for (player_id, player_hand, opponent_id, opponent_hand, game_status, _round_id) in match_results {
        // Fetch usernames for player and opponent
        let player_username = get_username(db_pool, player_id).await?;
        let opponent_username = get_username(db_pool, opponent_id).await?;

        // Convert hand to emoji
        let player_hand_emoji = hand_to_emoji(&player_hand);
        let opponent_hand_emoji = hand_to_emoji(&opponent_hand);

        // Construct the result message
        let result_message = match game_status.as_str() {
            "won" => format!("@{} played {} and won against @{}'s {}!", player_username, player_hand_emoji, opponent_username, opponent_hand_emoji),
            "lost" => format!("@{} played {} and lost against @{}'s {}!", player_username, player_hand_emoji, opponent_username, opponent_hand_emoji),
            "draw" => format!("@{} played {} against @{}'s {}. It's a draw!", player_username, player_hand_emoji, opponent_username, opponent_hand_emoji),
            _ => format!("@{} did not play a hand and lost!", player_username),
        };

        announcement.push_str(&result_message);
        announcement.push('\n');
    }

    Ok(announcement)
}

async fn get_username(db_pool: &DbPool, player_id: i64) -> Result<String, RusqliteError> {
    let conn = db_pool.get().expect("Failed to get DB connection");
    conn.query_row(
        "SELECT player_username FROM PlayerDetailsTable WHERE player_id = ?1",
        params![player_id],
        |row| row.get(0),
    )
}

fn hand_to_emoji(hand: &str) -> &str {
    match hand {
        "rock" => "🪨",
        "paper" => "📜",
        "scissors" => "✂️",
        "" => "🚫",
        _ => "❓",
    }
}

