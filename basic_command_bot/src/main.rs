use teloxide::{
    prelude::*,
    utils::command::BotCommands,
};
use chrono::{Local, DateTime};
mod admin;
use admin::{add_admin, remove_admin, list_admins, is_admin};

mod database;
use database::{init_db_pool, write_to_db, read_from_db};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let now: DateTime<Local> = Local::now();
    log::info!("Starting command bot...");
    log::info!("Starting timestamp: {}...", now.format("%Y-%m-%d %H:%M:%S %:z"));
    let db_pool = Arc::new(init_db_pool());     //let database_manager = DatabaseManager::new("my_database.db").expect("Failed to initialize the database manager");
    let bot = Bot::from_env();
    Command::repl_with(bot, move |bot, msg, cmd| answer(bot, msg, cmd, db_pool.clone())).await;

    //Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "displays a username.")]
    Username,
    #[command(description = "basic auth test.")]
    UsernameAndAge,
    #[command(description = "Write to sqllite db.")]
    Writesql,
    #[command(description = "Read from sqllite db.")]
    Readsql,
    //
    //AdminCommands
    //
    //#[command(description = "add a user to the admin list.")]
    #[command(description = "off")]
    AddAdmin(String),
    //#[command(description = "remove a user from the admin list.")]
    #[command(description = "off")]
    RemoveAdmin(String),
    //#[command(description = "list admin users.")]
    #[command(description = "off")]
    ListAdmins,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These 🌟Admin🌟 commands are supported:")]
enum AdminCommand {
    #[command(description = "add a user to the admin list.")]
    AddAdmin(String),
    #[command(description = "remove a user from the admin list.")]
    RemoveAdmin(String),
    #[command(description = "list admin users.")]
    ListAdmins,
}

async fn answer(bot: Bot,msg: Message, cmd: Command, db_pool: Arc<database::DbPool>) -> ResponseResult<()> {
    let _ = match cmd {
        //
        // basic commands
        //
        Command::Help => {
            let requester_username = msg.from().and_then(|user| user.username.clone()).unwrap_or_default();
            let is_admin = is_admin(&requester_username);
            if is_admin {
                bot.send_message(msg.chat.id, AdminCommand::descriptions().to_string()).await?;
            }
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            Ok::<(), Box<dyn std::error::Error>>(())
        }
        Command::Username => {
            if let Some(username) = msg.from().and_then(|user| user.username.clone()) {
                bot.send_message(msg.chat.id, format!("Your username is @{}.", username)).await?;
            } else {
                bot.send_message(msg.chat.id, "Unable to retrieve your username.").await?;
            }
            Ok::<(), Box<dyn std::error::Error>>(())    
        }
        Command::UsernameAndAge => {
            if let Some(requester_username) = msg.from().and_then(|user| user.username.clone()) {
                if requester_username != "juno0x153" {
                    bot.send_message(msg.chat.id, "You are not authorized to use this command.").await?;
                    } else {
                    bot.send_message(msg.chat.id, format!("Your username is valid.")).await?;
                }
            } else {
                bot.send_message(msg.chat.id, "Unable to retrieve your username.").await?;
            }
            Ok::<(), Box<dyn std::error::Error>>(())    
        }
        Command::Writesql => {
            let chat_id = msg.chat.id;
            // Respond to the user
            database::write_to_db("Some value").await.expect("Failed to write to the database");
            bot.send_message(msg.chat.id, "writing to database...").await?;
            Ok::<(), Box<dyn std::error::Error>>(())
        }
        Command::Readsql => {
            let value = database::read_from_db().await.expect("Failed to read from the database");
            bot.send_message(msg.chat.id, "reading from database...").await?;
            bot.send_message(msg.chat.id, format!("The value is {}.", value)).await?;
            Ok::<(), Box<dyn std::error::Error>>(())
        }
        //
        // admin commands
        //
        Command::AddAdmin(username) => {
            // user needs to be an admin. TODO need to set flag.
            let username = username.trim();
            if username.is_empty() {
                bot.send_message(msg.chat.id, "Please provide a non-empty username.").await?;
                return Ok(());
            } else if username.split_whitespace().count() != 1 {
                bot.send_message(msg.chat.id, "Only one username please, no spaces.").await?;
                return Ok(());
            } else if is_admin(&username) {
                bot.send_message(msg.chat.id, format!("@{} is already an admin.", username)).await?;
                return Ok(());
            } else { // Main case 
            add_admin(username.to_string().clone());
            bot.send_message(msg.chat.id, format!("Added @{} to admin list.", username)).await?;
            Ok(())
            }
        }

        Command::RemoveAdmin(username) => {
            // user needs to be an admin. TODO need to set flag.
            let username = username.trim();
            if username.is_empty() || !is_admin(&username) {
                if username.is_empty() {
                    bot.send_message(msg.chat.id, format!("Your command is empty, we need 1 username here.")).await?;
                } else if !is_admin(&username) {
                    bot.send_message(msg.chat.id, format!("User @{} is not in the admin list.", username)).await?;
                }
            } else {
                remove_admin(&username);
                bot.send_message(msg.chat.id, format!("Removed @{} from admin list.", username)).await?;
            }
            Ok(())
        }

        Command::ListAdmins => {
            // user needs to be an admin. TODO need to set flag.
            let admins = list_admins();
            bot.send_message(msg.chat.id, format!("Admins: {:?}", admins)).await?;
            Ok(())
        }
    };
    Ok(())
}


