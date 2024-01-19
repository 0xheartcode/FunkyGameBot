// basic_commands.rs

use teloxide::{prelude::*, utils::command::BotCommands};
use std::{error::Error};
use crate::enums::{Command, AdminCommand, DevCommand};
use crate::admin::{is_authorized_dev, is_authorized_sender};

//
//
//TODO BasicCommands
//
//====================================================

pub async fn help(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if is_authorized_dev(&msg) {
        bot.send_message(msg.chat.id, DevCommand::descriptions().to_string()).await?;
    }
    if is_authorized_sender(&msg) {
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
    bot.send_message(msg.chat.id, "The current version of the bot is v0.0.1.").await?;
    Ok(())
}

// TODO you have joined a game, you need to join the game with a hand
pub async fn join_game_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "You have joined the game. Or are trying to, this command is being built.").await?;
    Ok(())
}

// TODO you have created a game, you need to create the game with a hand
pub async fn create_game_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "You have created a game. Command being built.").await?;
    Ok(())
}

// TODO
pub async fn viewleaderboard_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Current leaderboard standings: ...").await?;
    Ok(())
}

// TODO
pub async fn viewleaderboard_specificseason_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "The leaderboard standings for a specific game: ...").await?;
    Ok(())
}
