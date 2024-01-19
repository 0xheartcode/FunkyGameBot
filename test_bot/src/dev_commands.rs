use teloxide::{prelude::*, utils::command::BotCommands};
use std::{error::Error, sync::Arc};
use crate::database::{DbPool, write_to_db, read_from_db};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These 🤖 Dev 🤖 commands are supported:")]
pub enum DevCommand {
    #[command(description = "Write to SQL database.")]
    Writesql(String),
    #[command(description = "Read from SQL database.")]
    Readsql,
    // ... other dev commands
}

pub fn handlers(db_pool: Arc<DbPool>) -> dptree::Handler<'static, (Bot, Message), Result<(), Box<dyn Error + Send + Sync>>> {
    use dptree::case;

    dptree::entry()
        .branch(case![DevCommand::Writesql(value)].endpoint(move |bot, msg, value| write_sql_command(bot, msg, db_pool.clone(), value)))
        .branch(case![DevCommand::Readsql].endpoint(move |bot, msg| read_sql_command(bot, msg, db_pool.clone())))
        // ... other dev command branches
}

async fn write_sql_command(bot: Bot, msg: Message, db_pool: Arc<DbPool>, value: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_dev(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    match write_to_db(&db_pool, &value).await {
        Ok(_) => bot.send_message(msg.chat.id, "Successfully written to database").await?,
        Err(e) => bot.send_message(msg.chat.id, format!("Error writing to database: {}", e)).await?,
    }
    Ok(())
}

async fn read_sql_command(bot: Bot, msg: Message, db_pool: Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_dev(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    match read_from_db(&db_pool).await {
        Ok(value) => bot.send_message(msg.chat.id, format!("Latest value from database: {}", value)).await?,
        Err(e) => bot.send_message(msg.chat.id, format!("Error reading from database: {}", e)).await?,
    }
    Ok(())
}

fn is_authorized_dev(msg: &Message) -> bool {
    // Implement your logic to check if the sender is an authorized developer
    false
}

