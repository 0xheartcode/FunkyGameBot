use teloxide::{prelude::*, utils::command::BotCommands};
use std::{error::Error};
use crate::admin_commands::AdminCommand;
use crate::dev_commands::DevCommand;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Register for a new game season.")]
    Signup,
    #[command(description = "Get the current version.")]
    Version,
}

pub fn handlers() -> dptree::Handler<'static, (Bot, Message), Result<(), Box<dyn Error + Send + Sync>>> {
    use dptree::case;

    dptree::entry()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Signup].endpoint(signup_command))
        .branch(case![Command::Version].endpoint(version_command))
}

async fn help(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let response = if is_authorized_dev(&msg) {
        format!("Developer commands:\n{}\n", DevCommand::descriptions())
    } else {
        String::new()
    } + &format!("Basic commands:\n{}", Command::descriptions());

    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}

async fn signup_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "You may want to sign up to a game. Bot is being built.").await?;
    Ok(())
}

async fn version_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "The current version of the bot is v0.0.1.").await?;
    Ok(())
}

fn is_authorized_dev(msg: &Message) -> bool {
    if let Some(true_sender_username) = msg.from().and_then(|user| user.username.as_ref()) {
        true_sender_username == "juno0x153" 
    } else {
        false
    }
}

