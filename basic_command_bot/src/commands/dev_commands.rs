// dev_commands.rs

use teloxide::{prelude::*};
use std::{error::Error};

use crate::admin::{is_authorized_dev};
use crate::database::{DbPool, write_to_db, read_from_db};

use std::sync::Arc;


//
//TODO DevCommands
//
//====================================================



pub async fn username_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_dev(&msg) { return Ok(());} //check is dev
                                                  //
                                                  //
    if let Some(username) = msg.from().and_then(|user| user.username.clone()) {
        bot.send_message(msg.chat.id, format!("Your username is @{}.", username)).await?;
    } else {
        bot.send_message(msg.chat.id, "Unable to retrieve your username.").await?;
    }
    Ok(())
}

pub async fn username_and_age_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_dev(&msg) { return Ok(());} //check is dev
                                                  //
                                                  //
    if let Some(requester_username) = msg.from().and_then(|user| user.username.clone()) {
        if requester_username != "juno0x153" {
            bot.send_message(msg.chat.id, "You are not authorized to use this command.").await?;
        } else {
            bot.send_message(msg.chat.id, "Your username is valid.").await?;
        }
    } else {
        bot.send_message(msg.chat.id, "Unable to retrieve your username.").await?;
    }
    Ok(())
}

pub async fn write_sql_command(bot: Bot, msg: Message, db_pool: Arc<DbPool>, value: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_dev(&msg) { return Ok(());} //check is dev
                                                  //
                                                  //
    match write_to_db(&db_pool, &value).await {
        Ok(_) => {
            if let Err(e) = bot.send_message(msg.chat.id, "Successfully written to database").await {
                log::error!("Failed to send message: {}", e);
            }
        },
        Err(e) => {
            if let Err(e) = bot.send_message(msg.chat.id, format!("Error writing to database: {}", e)).await {
                log::error!("Failed to send message: {}", e);
            }
        },
    }
    Ok(())
}

pub async fn read_sql_command(bot: Bot, msg: Message, db_pool: Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_dev(&msg) { return Ok(());} //check is dev
                                                  //
                                                  //
    match read_from_db(&db_pool).await {
        Ok(value) => {
            // Handle the case when reading from the database succeeds
            if let Err(e) = bot.send_message(msg.chat.id, format!("Latest value from database: {}", value)).await {
                // Log the error if sending the message fails
                log::error!("Failed to send message: {}", e);
            }
        },
        Err(e) => {
            // Handle the case when reading from the database fails
            if let Err(e) = bot.send_message(msg.chat.id, format!("Error reading from database: {}", e)).await {
                // Log the error if sending the message fails
                log::error!("Failed to send message: {}", e);
            }
        },
    }
    Ok(())
}


