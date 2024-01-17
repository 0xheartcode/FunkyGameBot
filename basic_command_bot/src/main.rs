use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};
use std::{env, error::Error};
use dotenv::dotenv;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

//use tokio_stream::wrappers::UnboundedReceiverStream;

use chrono::{Local, DateTime};
mod admin;
use admin::{add_admin, remove_admin, list_admins, is_admin};

mod database;
use database::{init_db_pool, DbPool, write_to_db, read_from_db};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let now: DateTime<Local> = Local::now();
    log::info!("Starting command bot...");
    log::info!("Starting timestamp: {}...", now.format("%Y-%m-%d %H:%M:%S %:z"));
    let db_pool = Arc::new(init_db_pool());     //let database_manager = DatabaseManager::new("my_database.db").expect("Failed to initialize the database manager");
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![db_pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Username].endpoint(username_command))
        .branch(case![Command::UsernameAndAge].endpoint(username_and_age_command))
        .branch(dptree::case![Command::Writesql(value)].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>, value: String| async move {
                write_sql_command(bot, msg, db_pool, value).await
        }))
        .branch(dptree::case![Command::Readsql].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
            read_sql_command(bot, msg, db_pool).await
        }))
        .branch(case![Command::ListAdmins].endpoint(list_admins_command))
        .branch(
            dptree::case![Command::AddAdmin(username)]
            .endpoint(|bot: Bot, msg: Message, username: String| async move {
                add_admin_command(bot, msg, username).await
            })
        )
        .branch(
            dptree::case![Command::RemoveAdmin(username)]
            .endpoint(|bot: Bot, msg: Message, username: String| async move {
                remove_admin_command(bot, msg, username).await
            })
        );

    let message_handler = Update::filter_message()
        .branch(command_handler);
    message_handler

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
    Writesql(String),
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


async fn help(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let requester_username = msg.from().and_then(|user| user.username.clone()).unwrap_or_default();
    let is_admin = is_admin(&requester_username);

    if is_admin {
        // Send admin command descriptions if the user is an admin.
        bot.send_message(msg.chat.id, AdminCommand::descriptions().to_string()).await?;
    }

    // Send general command descriptions to all users.
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;

    Ok(())
}

async fn username_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(username) = msg.from().and_then(|user| user.username.clone()) {
        bot.send_message(msg.chat.id, format!("Your username is @{}.", username)).await?;
    } else {
        bot.send_message(msg.chat.id, "Unable to retrieve your username.").await?;
    }
    Ok(())
}

async fn username_and_age_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
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

async fn write_sql_command(bot: Bot, msg: Message, db_pool: Arc<DbPool>, value: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    //let value = "Some value"; // Replace this with the actual value you want to write
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

async fn read_sql_command(bot: Bot, msg: Message, db_pool: Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
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


async fn add_admin_command(bot: Bot, msg: Message, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
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

async fn remove_admin_command(bot: Bot, msg: Message, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
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

async fn list_admins_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let admins = list_admins();
    bot.send_message(msg.chat.id, format!("Admins: {:?}", admins)).await?;
    Ok(())
}




