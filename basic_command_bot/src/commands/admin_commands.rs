/// admin_commands.rs

use teloxide::{prelude::*};
use std::{error::Error, sync::Arc};

use crate::admin::{
    is_authorized_sender, 
    list_admins, 
    is_admin, 
    remove_admin, 
    add_admin
};

use crate::database::{
    DbPool
};

use crate::commands::season::{start_new_season, stop_current_season, current_active_season, current_active_season_details};

//
//TODO AdminCommands
//
//====================================================


pub async fn add_admin_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    let username = username.trim();
    if username.is_empty() {
        bot.send_message(msg.chat.id, "Please provide a non-empty username.").await?;
    } else if username.split_whitespace().count() != 1 {
        bot.send_message(msg.chat.id, "Only one username please, no spaces.").await?;
    } else if is_admin(db_pool, &username)? {
        bot.send_message(msg.chat.id, format!("@{} is already an admin.", username)).await?;
    } else {
        add_admin(db_pool, username);
        bot.send_message(msg.chat.id, format!("Added @{} to admin list.", username)).await?;
    }
    Ok(())
}

pub async fn remove_admin_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    let username = username.trim();
    if username.is_empty() || !is_admin(db_pool, &username).unwrap_or(false) {
        if username.is_empty() {
            bot.send_message(msg.chat.id, "Your command is empty, we need 1 username here.").await?;
        } else {
            bot.send_message(msg.chat.id, format!("User @{} is not in the admin list.", username)).await?;
        }
    } else {
        remove_admin(db_pool, &username);
        bot.send_message(msg.chat.id, format!("Removed @{} from admin list.", username)).await?;
    }
    Ok(())
}

pub async fn list_admins_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    let admins = list_admins(db_pool)?;
    let mut response = String::from("Admins:\n");
    for admin in admins {
    response.push_str(&format!("@{}\n", admin));
    }

    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}


// TODO GOAL A

pub async fn start_new_season_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, season_info: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    
    let season_info = season_info.trim();
    if season_info.split_whitespace().count() != 2 {
        bot.send_message(msg.chat.id, "The command should be used like this '/startnewseason <Title> <Number of Players>'.").await?;
        return Ok(());
    }

    let mut parts = season_info.split_whitespace();

    // Get the first part as the season_name
    let season_name = match parts.next() {
        Some(name) => name.to_string(),  // Convert the &str slice to a String
        None => {
            bot.send_message(msg.chat.id, "Please provide a season name.").await?;
            return Ok(());
        }
    };

    // Get the last part as max_players
    let max_players: i32 = match parts.next() {  // Changed from parts.last() to parts.next()
        Some(num_str) => match num_str.parse() {
            Ok(num) => num,
            Err(_) => {
                bot.send_message(msg.chat.id, "Invalid number format for max players.").await?;
                return Ok(());
            }
        },
        None => {
            bot.send_message(msg.chat.id, "Please provide the maximum number of players.").await?;
            return Ok(());
        }
    };
       // Check if there is an active season
    match current_active_season(db_pool).await {
        Ok(Some(active_season)) => {
            // There is an active season, send a message
            bot.send_message(msg.chat.id, format!("A season is already in progress: '{}'. Another season cannot be started until the current one is concluded.", active_season)).await?;
        },
        Ok(None) => {
            // No active season, proceed to start a new one
            match start_new_season(db_pool, &season_name, max_players).await {
                Ok(_) => {
                    // Successfully started a new season
                    bot.send_message(msg.chat.id, format!("A new rock-paper-scissors season '{}' has started! Maximum players allowed: {}. Let the games begin.", season_name, max_players)).await?;
                },
                Err(e) => {
                    // Error in starting a new season
                    bot.send_message(msg.chat.id, format!("Failed to start new season '{}': {}", season_name, e)).await?;
                }
            }
        },
        Err(e) => {
            // Error in checking for an active season
            bot.send_message(msg.chat.id, format!("Error checking for active season: {}", e)).await?;
        }
    }

    Ok(())
}

pub async fn stop_new_season_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The current rock-paper-scissors season has been concluded. Stay tuned for results and rewards.").await?;
    Ok(())
}


pub async fn current_season_status_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Check if there is an active season
    match current_active_season_details(db_pool).await {
        Ok(Some((name, start_date, max_players))) => {
            let message = format!(
                "Current active season: '{}'\nStarted on: {}\nMax players: {}",
                name, start_date, max_players
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



// TODO GOAL B

pub async fn startsignupphase_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The signup phase for the new rock-paper-scissors season is now open. Interested players can register.").await?; 
   Ok(())
}

pub async fn stopsignupphase_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
   bot.send_message(msg.chat.id, "The signup phase is now closed. Preparations for the game will now commence.").await?; 
    Ok(())
}


// TODO GOAL C


pub async fn startgamingphase_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The gaming phase has begun! Players, get ready to challenge each other.").await?;
    Ok(())
}

pub async fn stopgamingphase_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The gaming phase has ended. Thank you to all participants!").await?;
    Ok(())
}

pub async fn start_round_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Finally, let us start the round !").await?;
    Ok(())
}

pub async fn stop_round_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Round is over, everyone back to their corner !").await?;
    Ok(())
}

// TODO GOAL D


pub async fn approveplayer_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Player has been successfully approved for participation.").await?;
    Ok(())
}

pub async fn refuseplayer_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Player's request to participate has been refused.").await?;
    Ok(())
}

pub async fn view_signuplist_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Here's the list of players who have signed up: ...").await?;
    Ok(())
}

pub async fn view_approved_list_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Here's the list of approved players: ...").await?;
    Ok(())
}

pub async fn viewrefusedlist_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Here's the list of players whose requests were refused: ...").await?;
    Ok(())
}

//
// TODO Broadcast and Group messages
//
//

pub async fn set_broadcastchannel_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "This is a command to set the broadcast channel").await?;
    Ok(())
}

pub async fn set_group_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "This is a command to set the group channel").await?;
    Ok(())
}

pub async fn msg_broadcastchannel_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Send a message to the broadcast channel").await?;
    Ok(())
}

pub async fn msg_group_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Send a message to the group channel").await?;
    Ok(())
}


pub async fn get_group_broadcast_id_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Retrieving the group and broadcast channel ID if set").await?;
    Ok(())
}


pub async fn reset_group_broadcast_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Reseting group and Broadcast channel. Global messages will not work anymore.").await?;
    Ok(())
}




