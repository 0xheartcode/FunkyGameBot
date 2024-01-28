/// basic_commands.rs

use teloxide::{prelude::*, utils::command::BotCommands};
use std::{error::Error, sync::Arc};
use crate::enums::{Command, AdminCommand, DevCommand};
use crate::admin::{is_authorized_dev, is_authorized_sender};

use crate::database::{DbPool};

use crate::commands::season::{
    current_active_season_details,
};
use rusqlite::{params };


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
pub async fn viewleaderboard_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Current leaderboard standings: ...").await?;
    Ok(())
}

// TODO
pub async fn playrock_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Playing the rock hand 🪨.").await?;
    Ok(())
}

// TODO
pub async fn playpaper_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Playing the paper hand 📜.").await?;
    Ok(())
}

// TODO
pub async fn playscissors_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Playing the scissors hand ✂.").await?;
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


