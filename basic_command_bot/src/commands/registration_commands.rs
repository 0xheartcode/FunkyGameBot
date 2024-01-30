/// registration_commands.rs

use teloxide::{prelude::* };
use std::{error::Error, sync::Arc};
use crate::database::{DbPool};
use rusqlite::{params, Error as RusqliteError};

use crate::admin::{
    is_authorized_sender, 
};

use crate::commands::season:: {
    current_active_season_id,
    current_active_season_details
};



pub async fn approveplayer_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, player_username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    //let player_username = player_username.trim();
    if player_username.split_whitespace().count() != 1 {
        bot.send_message(msg.chat.id, "The command should be used like this '/approveplayer <username>'. Make sure the player username is correct and pending.").await?;
        return Ok(());
    }

    let season_details = current_active_season_details(db_pool).await?;
    if let Some((season_id, season_name, _, _, status)) = season_details {
        if status != "start_signup" {
            bot.send_message(msg.chat.id, "Approvals are only allowed during the 'start_signup' phase.").await?;
            return Ok(());
        }

        let (response_message, player_id) = update_player_status_to_accepted(db_pool, season_id, &player_username).await?;

        bot.send_message(msg.chat.id, response_message).await?;

        // Send a message to the player if their ID is not 0
        if player_id != 0 {
            let acceptance_message = format!("Your registration to the new game {} has been accepted!", season_name);
            bot.send_message(ChatId(player_id.into()), acceptance_message).await?;
        }
    } else {
        bot.send_message(msg.chat.id, "No active season found.").await?;
    }

    Ok(())
}

pub async fn refuseplayer_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, player_username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    let player_username = player_username.trim();
    if player_username.split_whitespace().count() != 1 {  
        bot.send_message(msg.chat.id, "The command should be used like this '/refuseplayer <username>'. Make sure the player username is correct and pending.").await?;
        return Ok(());
    }

    let season_details = current_active_season_details(db_pool).await?;
    if let Some((season_id, _, _, _, status)) = season_details {
        if status != "start_signup" {
            bot.send_message(msg.chat.id, "Refusals are only allowed during the 'start_signup' phase.").await?;
            return Ok(());
        }

        // Call the new function to update player status and get the response message
        let response_message = update_player_status_to_refused(db_pool, season_id, &player_username).await?;
        bot.send_message(msg.chat.id, response_message).await?;
    } else {
        bot.send_message(msg.chat.id, "No active season found.").await?;
    }
    Ok(())
}

pub async fn update_player_status_to_accepted(db_pool: &DbPool, season_id: i32, player_username: &str) -> Result<(String, i32), RusqliteError> {
    let mut conn = db_pool.get().expect("Failed to get DB connection");
    let tx = conn.transaction()?;

    // Update player status to 'accepted' in MasterCandidateTable
    let rows_updated = tx.execute(
        "UPDATE MasterCandidateTable SET player_status = 'accepted' WHERE season_id = ?1 AND player_username = ?2 AND player_status = 'pending'",
        params![season_id, player_username],
    )?;

    if rows_updated > 0 {
        // Fetch the player_id for the accepted player
        let player_id: i32 = tx.query_row(
            "SELECT player_id FROM MasterCandidateTable WHERE season_id = ?1 AND player_username = ?2",
            params![season_id, player_username],
            |row| row.get(0),
        )?;

        // Insert the accepted player into PlayerDetailsTable
        tx.execute(
            "INSERT INTO PlayerDetailsTable (season_id, player_id, player_username, score) VALUES (?1, ?2, ?3, 0)",
            params![season_id, player_id, player_username],
        )?;

        tx.commit()?;
        Ok((format!("Player '{}' has been accepted for participation.", player_username), player_id))
    } else {
        Ok(("No pending player found with the given username for the current season.".to_string(), 0))
    }
}




pub async fn update_player_status_to_refused(db_pool: &DbPool, season_id: i32, player_username: &str) -> Result<String, RusqliteError> {
    let conn = db_pool.get().expect("Failed to get DB connection");
    let rows_updated = conn.execute(
        "UPDATE MasterCandidateTable SET player_status = 'refused' WHERE season_id = ?1 AND player_username = ?2 AND player_status = 'pending'",
        params![season_id, player_username],
    )?;

    Ok(if rows_updated > 0 {
        format!("Player '{}' has been refused participation.", player_username)
    } else {
        "No pending player found with the given username for the current season.".to_string()
    })
}






pub async fn view_signuplist_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Get the current active season ID
    let current_season_id = match current_active_season_id(db_pool).await? {
        Some(id) => id,
        None => {
            bot.send_message(msg.chat.id, "No active season found.").await?;
            return Ok(());
        }
    };

    let signup_list = get_signup_list_for_season(db_pool, current_season_id, "all").await?;

    bot.send_message(msg.chat.id, signup_list).await?;
    Ok(())
}


pub async fn view_approved_list_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Get the current active season ID
    let current_season_id = match current_active_season_id(db_pool).await? {
        Some(id) => id,
        None => {
            bot.send_message(msg.chat.id, "No active season found.").await?;
            return Ok(());
        }
    };

    let signup_list = get_signup_list_for_season(db_pool, current_season_id, "accepted").await?;

    bot.send_message(msg.chat.id, signup_list).await?;
    Ok(())
}




pub async fn viewrefusedlist_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Get the current active season ID
    let current_season_id = match current_active_season_id(db_pool).await? {
        Some(id) => id,
        None => {
            bot.send_message(msg.chat.id, "No active season found.").await?;
            return Ok(());
        }
    };

    let signup_list = get_signup_list_for_season(db_pool, current_season_id, "refused").await?;

    bot.send_message(msg.chat.id, signup_list).await?;
    Ok(())
}





pub async fn get_signup_list_for_season(pool: &DbPool, season_id: i32, status_filter: &str) -> Result<String, RusqliteError> {
    let conn = pool.get().expect("Failed to get DB connection");
    // call this function either with "all" or "pending" or "accepted" or "refused"
    // Prepare the SQL query based on the status filter
    let query = match status_filter {
        "all" => "SELECT player_username, player_status FROM MasterCandidateTable WHERE season_id = ?1",
        _ => "SELECT player_username, player_status FROM MasterCandidateTable WHERE season_id = ?1 AND player_status = ?2",
    };

    let mut stmt = conn.prepare(query)?;

    // Execute the query with or without the status filter
    let mut rows = match status_filter {
        "all" => stmt.query(params![season_id])?,
        _ => stmt.query(params![season_id, status_filter])?,
    };

    let mut response = "List of players who have signed up:\n".to_string();
    while let Some(row) = rows.next()? {
        let player_username: String = row.get(0)?;
        let player_status: String = row.get(1)?;
        response.push_str(&format!("@{} - {}\n", player_username, player_status));
    }

    if response.ends_with("\n") {
        response.pop(); // Remove the trailing newline
    }

    if response == "List of players who have signed up:" {
        if status_filter == "all" {
            response = "No player candidates have signed up yet.".to_string();
        } else {
            response = format!("No player candidates found with status '{}'.", status_filter);
        }
    }

    Ok(response)
}
