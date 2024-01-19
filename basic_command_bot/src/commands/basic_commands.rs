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


