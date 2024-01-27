/// grp_broadcast_commands.rs

use teloxide::{prelude::* };
use std::{error::Error, sync::Arc};
use crate::database::{DbPool};
use rusqlite::{Error as RusqliteError};

use crate::admin::{
    is_authorized_sender, 
};



// Add the necessary imports and any additional dependencies you might need

pub async fn set_broadcast_channel_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, channel_id: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check if the user is authorized
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());
    }

    let channel_id = channel_id.trim();
    if channel_id.split_whitespace().count() != 1 || !channel_id.chars().all(char::is_numeric) {
        bot.send_message(msg.chat.id, "The command should be used like this '/setbroadcastchannel <ChannelID>'. Make sure the ChannelID is a numeric value. Contact the dev if you need help.").await?;
        return Ok(());
    }

    // Call the database function to set the channel ID
    match set_broadcast_channel_id(db_pool, &channel_id).await {
        Ok(_) => bot.send_message(msg.chat.id, format!("Broadcast channel set to: {}", channel_id)).await?,
        Err(e) => bot.send_message(msg.chat.id, format!("Failed to set broadcast channel: {}", e)).await?,
    };

    Ok(())
}

pub async fn set_group_channel_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, channel_id: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check if the user is authorized
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());
    }

    let channel_id = channel_id.trim();
    if channel_id.split_whitespace().count() != 1 || !channel_id.chars().all(char::is_numeric) {
        bot.send_message(msg.chat.id, "The command should be used like this '/setgroupchannel <ChannelID>'. Make sure the ChannelID is a numeric value. Contact the dev if you need help.").await?;
        return Ok(());
    }

    // Call the database function to set the channel ID
    match set_group_channel_id(db_pool, &channel_id).await {
        Ok(_) => bot.send_message(msg.chat.id, format!("Group channel set to: {}", channel_id)).await?,
        Err(e) => bot.send_message(msg.chat.id, format!("Failed to set group channel: {}", e)).await?,
    };

    Ok(())
}

pub async fn get_group_broadcast_id_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check if the user is authorized
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());
    }

    // Retrieve the channel IDs from the database
    match get_group_broadcast_ids(db_pool).await {
        Ok((broadcast_id, group_id)) => {
            let message = format!(
                "Broadcast Channel ID: {:?}\nGroup Channel ID: {:?}",
                broadcast_id.unwrap_or_else(|| "Not set".into()),
                group_id.unwrap_or_else(|| "Not set".into())
            );
            bot.send_message(msg.chat.id, message).await?;
        },
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to retrieve channel IDs: {}", e)).await?;
        }
    }

    Ok(())
}

pub async fn reset_group_broadcast_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check if the user is authorized
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());
    }

    // Reset the channel settings in the database
    match reset_group_broadcast(db_pool).await {
        Ok(_) => {
            bot.send_message(msg.chat.id, "Group and broadcast channel settings have been reset.").await?;
        },
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to reset channel settings: {}", e)).await?;
        }
    }

    Ok(())
}


// Function to set the broadcast channel ID
pub async fn set_broadcast_channel_id(pool: &DbPool, channel_id: &str) -> Result<(), RusqliteError>  {
    let conn = pool.get().expect("Failed to get connection from pool");
    let num_rows_updated = conn.execute("UPDATE channel_settings SET broadcast_channel_id = ?1 WHERE id = 1", [channel_id])?;
    println!("Number of rows updated: {}", num_rows_updated);
    Ok(())
}

// Function to set the group channel ID
pub async fn set_group_channel_id(pool: &DbPool, channel_id: &str) -> Result<(), RusqliteError>  {
    let conn = pool.get().expect("Failed to get connection from pool");
    let num_rows_updated = conn.execute("UPDATE channel_settings SET group_channel_id = ?1 WHERE id = 1", [channel_id])?;
    println!("Number of rows updated: {}", num_rows_updated);
    Ok(())
}

// Function to get the current group and broadcast channel IDs
pub async fn get_group_broadcast_ids(pool: &DbPool) -> Result<(Option<String>,Option<String>), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let mut stmt = conn.prepare("SELECT broadcast_channel_id, group_channel_id FROM channel_settings ORDER BY id DESC LIMIT 1")?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        Ok((row.get(0)?, row.get(1)?))
    } else {
        Ok((None, None))
    }
}

// Function to reset the group and broadcast channel settings
pub async fn reset_group_broadcast(pool: &DbPool) -> Result<(), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    conn.execute("UPDATE channel_settings SET broadcast_channel_id = NULL, group_channel_id = NULL WHERE id = 1", [])?;
    Ok(())
}

pub async fn msg_broadcastchannel_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, message_text: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    // TODO no safety here. Doesn't say if it does not work. 
    let (broadcast_channel_id, _) = get_group_broadcast_ids(db_pool).await?;
    if let Some(channel_id_str) = broadcast_channel_id {
        let mut channel_id: i64 = channel_id_str.parse()?;
        channel_id *= -1; // Make it negative
        bot.send_message(ChatId(channel_id), message_text).await?;
        bot.send_message(msg.chat.id, "Sends a message to the broadcast channel").await?;
    } else {
        bot.send_message(msg.chat.id, "Broadcast channel is not set.").await?;
    }

    Ok(())
}

pub async fn msg_group_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, message_text: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    // Retrieve the group channel ID from the database
    let (_, group_channel_id) = get_group_broadcast_ids(db_pool).await?;
    
    if let Some(channel_id_str) = group_channel_id {
        // Parse the channel ID string as an i64
        let mut channel_id: i64 = channel_id_str.parse()?;
        channel_id *= -1;

        // Send the message to the group channel
        bot.send_message(ChatId(channel_id), message_text.clone()).await?;
        bot.send_message(msg.chat.id, "Sent the message to the group channel").await?;
    } else {
        bot.send_message(msg.chat.id, "Group channel is not set.").await?;
    }

    Ok(())
}
