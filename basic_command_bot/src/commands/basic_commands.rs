/// basic_commands.rs

use teloxide::{prelude::*, utils::command::BotCommands};
use std::{error::Error, sync::Arc};
use crate::enums::{Command, AdminCommand, DevCommand};
use crate::admin::{is_authorized_dev, is_authorized_sender};

use crate::database::{DbPool};

use crate::commands::season::{
    current_active_season_details,
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

// TODO implement function
pub async fn signup_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "You may want to sign up to a game. Bot is being built.").await?;
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

pub async fn status_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Check if there is an active season
    match current_active_season_details(db_pool).await {
        Ok(Some((name, start_date, max_players, status))) => {
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


