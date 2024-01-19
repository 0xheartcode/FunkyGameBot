// admin_commands.rs

use teloxide::{prelude::*, utils::command::BotCommands};
use std::{error::Error};
use crate::enums::{Command, AdminCommand, DevCommand};
use crate::admin::{is_authorized_sender, list_admins, is_admin, remove_admin, add_admin};

//
//TODO AdminCommands
//
//====================================================


pub async fn add_admin_command(bot: Bot, msg: Message, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }

    let username = username.trim();
    if username.is_empty() {
        bot.send_message(msg.chat.id, "Please provide a non-empty username.").await?;
    } else if username.split_whitespace().count() != 1 {
        bot.send_message(msg.chat.id, "Only one username please, no spaces.").await?;
    } else if is_admin(&username) {
        bot.send_message(msg.chat.id, format!("@{} is already an admin.", username)).await?;
    } else {
        add_admin(username.to_string());
        bot.send_message(msg.chat.id, format!("Added @{} to admin list.", username)).await?;
    }
    Ok(())
}

pub async fn remove_admin_command(bot: Bot, msg: Message, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }

    let username = username.trim();
    if username.is_empty() || !is_admin(&username) {
        if username.is_empty() {
            bot.send_message(msg.chat.id, "Your command is empty, we need 1 username here.").await?;
        } else {
            bot.send_message(msg.chat.id, format!("User @{} is not in the admin list.", username)).await?;
        }
    } else {
        remove_admin(&username);
        bot.send_message(msg.chat.id, format!("Removed @{} from admin list.", username)).await?;
    }
    Ok(())
}

pub async fn list_admins_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    let admins = list_admins(); // Assuming this returns a Vec<String> or similar
    let mut response = String::from("Admins:\n");
    for admin in admins {
        response.push_str(&format!("@{}\n", admin));
    }

    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}


pub async fn startnewseason_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "A new rock-paper-scissors season has started! Let the games begin.").await?;
    Ok(())
}

pub async fn stopnewseason_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The current rock-paper-scissors season has been concluded. Stay tuned for results and rewards.").await?;
    Ok(())
}

pub async fn startsignupphase_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The signup phase for the new rock-paper-scissors season is now open. Interested players can register.").await?; 
   Ok(())
}

pub async fn stopsignupphase_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
   bot.send_message(msg.chat.id, "The signup phase is now closed. Preparations for the game will now commence.").await?; 
    Ok(())
}

pub async fn startgamingphase_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The gaming phase has begun! Players, get ready to challenge each other.").await?;
    Ok(())
}

pub async fn stopgamingphase_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The gaming phase has ended. Thank you to all participants!").await?;
    Ok(())
}

pub async fn approveplayer_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Player has been successfully approved for participation.").await?;
    Ok(())
}

pub async fn refuseplayer_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Player's request to participate has been refused.").await?;
    Ok(())
}

pub async fn viewsignuplist_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Here's the list of players who have signed up: ...").await?;
    Ok(())
}

pub async fn viewapprovedlist_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Here's the list of approved players: ...").await?;
    Ok(())
}

pub async fn viewrefusedlist_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Here's the list of players whose requests were refused: ...").await?;
    Ok(())
}

pub async fn viewleaderboard_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Current leaderboard standings: ...").await?;
    Ok(())
}
