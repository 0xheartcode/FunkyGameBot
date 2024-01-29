/// basic_commands.rs

use teloxide::{prelude::*, utils::command::BotCommands};
use std::{error::Error, sync::Arc};
use crate::enums::{Command, AdminCommand, DevCommand};
use crate::admin::{is_authorized_dev, is_authorized_sender};

use crate::database::{DbPool};

use crate::commands::season::{
    current_active_season_details,
    current_active_season_id,
};
use rusqlite::{params };


use crate::commands::playing_commands::{
    insert_player_hand_choice,
    current_game_status_and_season_id,
    check_player_in_game,
    get_current_round_id,
    fetch_leaderboard,
    prepare_leaderboard_string,
};


//
//
//TODO BasicCommands
//
//====================================================


pub async fn help(bot: Bot, msg: Message, db_pool: Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if is_authorized_dev(&msg) {
        bot.send_message(msg.chat.id, DevCommand::descriptions().to_string()).await?;
    }
    if is_authorized_sender(&msg, &db_pool) {
        bot.send_message(msg.chat.id, AdminCommand::descriptions().to_string()).await?;
    }
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

pub async fn signup_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check if there is an active season in the "start_signup" phase
    let season_details = current_active_season_details(db_pool).await?;
    if let Some((season_id, _, _, _, status)) = season_details {
        if status == "start_signup" {
            // Extract player details
            let player_id = msg.from().expect("Message has no sender").id.0;
            let player_username = msg.from()
                                     .expect("Message has no sender")
                                     .username
                                     .as_ref() // Borrow the contents of the Option
                                     .unwrap_or(&"unknown".to_string())
                                     .to_string();

            // Insert player into MasterCandidateTable
            let conn = db_pool.get().expect("Failed to get DB connection");

            // Check if the player is already signed up
            let exists: i64 = conn.query_row(
                "SELECT COUNT(*) FROM MasterCandidateTable WHERE season_id = ?1 AND player_id = ?2",
                params![season_id, player_id],
                |row| row.get(0),
            ).unwrap_or(0);

            if exists != 0 {
                // If a player already exists in MasterCandidateTable, abort.
                bot.send_message(msg.chat.id, "You are already on the waitinglist for this game.").await?;
                return Ok(());
            }

            conn.execute(
                "INSERT INTO MasterCandidateTable (season_id, player_id, player_username, player_wallet, player_status) VALUES (?1, ?2, ?3, '', 'pending')",
                params![season_id, player_id, player_username],
            )?;

            bot.send_message(msg.chat.id, "You have successfully signed up to the waitinglist for the game!").await?;
        } else {
            bot.send_message(msg.chat.id, "Signups are currently closed.").await?;
        }
    } else {
        bot.send_message(msg.chat.id, "There is no active season currently.").await?;
    }

    Ok(())
}

// TODO enable checking the Cargo.toml file for version.
pub async fn version_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "The current version of the bot is v0.0.2.").await?;
    Ok(())
}


// TODO
pub async fn viewleaderboard_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let current_season_id = current_active_season_id(db_pool).await?;

    // Check if a current season ID is present and fetch the leaderboard accordingly
    match current_season_id {
        Some(season_id) => {
            match fetch_leaderboard(db_pool, season_id).await {
                Ok(leaderboard) => {
                    let response = prepare_leaderboard_string(leaderboard).await;
                    bot.send_message(msg.chat.id, &response).await?;
                },
                Err(_) => {
                    bot.send_message(msg.chat.id, "Failed to fetch the leaderboard.").await?;
                }
            }
        },
        None => {
            bot.send_message(msg.chat.id, "Leaderboards only work during active games. Check the main channel for logs").await?;
        }
    }
    Ok(())
}


pub async fn playrock_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check if there is an active season in the "round_ongoing" phase and get the season_id
    let game_status_and_season_id = current_game_status_and_season_id(db_pool).await?;
    if let Some((game_status, season_id)) = game_status_and_season_id {
        if game_status != "round_ongoing" {
            bot.send_message(msg.chat.id, "There is no round currently ongoing.").await?;
            return Ok(());
        }

        // Extract player details
        let player_id = msg.from().expect("Message has no sender").id.0;

        // Check if the player is in the current game
        let player_in_game = check_player_in_game(db_pool, player_id.try_into().unwrap(), season_id).await?;
        if !player_in_game {
            bot.send_message(msg.chat.id, "You are not part of the current game.").await?;
            return Ok(());
        }

        // Get the current round ID
        let round_id = get_current_round_id(db_pool, season_id).await?;
        if let Some(current_round_id) = round_id {
            // Insert the player's choice into the RoundDetailsTable
            let success = insert_player_hand_choice(db_pool, current_round_id, player_id.try_into().unwrap(), "rock").await?;
            if success {
                bot.send_message(msg.chat.id, "Playing the rock hand 🪨.").await?;
            } else {
                bot.send_message(msg.chat.id, "You have already played this round.").await?;
            }
        } else {
            bot.send_message(msg.chat.id, "No active round found.").await?;
        }
    } else {
        bot.send_message(msg.chat.id, "No active season found.").await?;
    }

    Ok(())
}


pub async fn playpaper_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check if there is an active season in the "round_ongoing" phase and get the season_id
    let game_status_and_season_id = current_game_status_and_season_id(db_pool).await?;
    if let Some((game_status, season_id)) = game_status_and_season_id {
        if game_status != "round_ongoing" {
            bot.send_message(msg.chat.id, "There is no round currently ongoing.").await?;
            return Ok(());
        }

        // Extract player details
        let player_id = msg.from().expect("Message has no sender").id.0;

        // Check if the player is in the current game
        let player_in_game = check_player_in_game(db_pool, player_id.try_into().unwrap(), season_id).await?;
        if !player_in_game {
            bot.send_message(msg.chat.id, "You are not part of the current game.").await?;
            return Ok(());
        }

        // Get the current round ID
        let round_id = get_current_round_id(db_pool, season_id).await?;
        if let Some(current_round_id) = round_id {
            // Insert the player's choice into the RoundDetailsTable
            let success = insert_player_hand_choice(db_pool, current_round_id, player_id.try_into().unwrap(), "paper").await?;
            if success {
                bot.send_message(msg.chat.id, "Playing the paper hand 📜.").await?;
            } else {
                bot.send_message(msg.chat.id, "You have already played this round.").await?;
            }
        } else {
            bot.send_message(msg.chat.id, "No active round found.").await?;
        }
    } else {
        bot.send_message(msg.chat.id, "No active season found.").await?;
    }

    Ok(())
}


pub async fn playscissors_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check if there is an active season in the "round_ongoing" phase and get the season_id
    let game_status_and_season_id = current_game_status_and_season_id(db_pool).await?;
    if let Some((game_status, season_id)) = game_status_and_season_id {
        if game_status != "round_ongoing" {
            bot.send_message(msg.chat.id, "There is no round currently ongoing.").await?;
            return Ok(());
        }

        // Extract player details
        let player_id = msg.from().expect("Message has no sender").id.0;

        // Check if the player is in the current game
        let player_in_game = check_player_in_game(db_pool, player_id.try_into().unwrap(), season_id).await?;
        if !player_in_game {
            bot.send_message(msg.chat.id, "You are not part of the current game.").await?;
            return Ok(());
        }

        // Get the current round ID
        let round_id = get_current_round_id(db_pool, season_id).await?;
        if let Some(current_round_id) = round_id {
            // Insert the player's choice into the RoundDetailsTable
            let success = insert_player_hand_choice(db_pool, current_round_id, player_id.try_into().unwrap(), "scissors").await?;
            if success {
                bot.send_message(msg.chat.id, "Playing the scissors hand ✂.").await?;
            } else {
                bot.send_message(msg.chat.id, "You have already played this round.").await?;
            }
        } else {
            bot.send_message(msg.chat.id, "No active round found.").await?;
        }
    } else {
        bot.send_message(msg.chat.id, "No active season found.").await?;
    }

    Ok(())
}


pub async fn status_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {

    // Check if there is an active season
    match current_active_season_details(db_pool).await {
        Ok(Some((_, name, start_date, max_players, status))) => {
            let message = format!(
                "Current active season: '{}'\nStarted on: {}\nMax players: {}\nStatus: {}",
                name, start_date, max_players, status
            );
            bot.send_message(msg.chat.id, message).await?;
        },
        Ok(None) => {
            bot.send_message(msg.chat.id, "There is no active season currently.").await?;
        },
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to get current season status: {}", e)).await?;
        }
    }

    Ok(())
}


