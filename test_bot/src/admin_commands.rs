use teloxide::{prelude::*, utils::command::BotCommands};
use std::{error::Error};
use crate::admin::{add_admin, remove_admin, list_admins, is_admin};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These 🌟 Admin 🌟 commands are supported:")]
pub enum AdminCommand {
    #[command(description = "add a user to the admin list.")]
    AddAdmin(String),
    #[command(description = "remove a user from the admin list.")]
    RemoveAdmin(String),
    #[command(description = "list admin users.")]
    ListAdmins,
    #[command(description = "Start a new season for the game.")]
    StartNewSeason,
    #[command(description = "Stop the current season of the game.")]
    StopNewSeason,
    // ... other admin commands
}

pub fn handlers() -> dptree::Handler<'static, (Bot, Message), Result<(), Box<dyn Error + Send + Sync>>> {
    use dptree::case;

    dptree::entry()
        .branch(case![AdminCommand::AddAdmin(username)].endpoint(add_admin_command))
        .branch(case![AdminCommand::RemoveAdmin(username)].endpoint(remove_admin_command))
        .branch(case![AdminCommand::ListAdmins].endpoint(list_admins_command))
        .branch(case![AdminCommand::StartNewSeason].endpoint(start_new_season_command))
        .branch(case![AdminCommand::StopNewSeason].endpoint(stop_new_season_command))
        // ... other admin command branches
}

// Implementations of admin commands functions
async fn add_admin_command(bot: Bot, msg: Message, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    // Implementation of adding an admin
    Ok(())
}

async fn remove_admin_command(bot: Bot, msg: Message, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    // Implementation of removing an admin
    Ok(())
}

async fn list_admins_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    // Implementation of listing admins
    Ok(())
}

async fn start_new_season_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "A new season has started!").await?;
    Ok(())
}

async fn stop_new_season_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The current season has been concluded.").await?;
    Ok(())
}

fn is_authorized_sender(msg: &Message) -> bool {
    // Implementation to check if the sender is authorized
    false
}

