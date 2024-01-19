use teloxide::{
    dispatching::{UpdateHandler},
    //dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
};
use std::{error::Error};
//use dotenv::dotenv;
//type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//use tokio_stream::wrappers::UnboundedReceiverStream;

use chrono::{Local, DateTime};
mod admin;
use admin::{add_admin, remove_admin, list_admins, is_admin};

mod database;
use database::{init_db_pool, DbPool};
use std::sync::Arc;

mod enums;
use enums::{Command, AdminCommand, DevCommand};

mod commands;
use commands::basic_commands::{help, signup_command, version_command};
use commands::dev_commands::{username_command, username_and_age_command, write_sql_command, read_sql_command,  };

use crate::admin::{is_authorized_sender};

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
        //
        //BasicCommands
        //
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Signup].endpoint(signup_command))
        .branch(case![Command::Version].endpoint(version_command))
        //
        //DevCommands
        //
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
        //
        //AdminCommands
        //
        .branch(case![Command::StartNewSeason].endpoint(startnewseason_command))
        .branch(case![Command::StopNewSeason].endpoint(stopnewseason_command))
        .branch(case![Command::StartSignupPhase].endpoint(startsignupphase_command))
        .branch(case![Command::StopSignupPhase].endpoint(stopsignupphase_command))
        .branch(case![Command::StartGamingPhase].endpoint(startgamingphase_command))
        .branch(case![Command::StopGamingPhase].endpoint(stopgamingphase_command))
        .branch(case![Command::ApprovePlayer].endpoint(approveplayer_command))
        .branch(case![Command::RefusePlayer].endpoint(refuseplayer_command))
        .branch(case![Command::ViewSignupList].endpoint(viewsignuplist_command))
        .branch(case![Command::ViewApprovedList].endpoint(viewapprovedlist_command))
        .branch(case![Command::ViewRefusedList].endpoint(viewrefusedlist_command))
        .branch(case![Command::ViewLeaderboard].endpoint(viewleaderboard_command))
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





//
//TODO AdminCommands
//
//====================================================




async fn add_admin_command(bot: Bot, msg: Message, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
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

async fn remove_admin_command(bot: Bot, msg: Message, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
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

async fn list_admins_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
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


async fn startnewseason_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "A new rock-paper-scissors season has started! Let the games begin.").await?;
    Ok(())
}

async fn stopnewseason_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The current rock-paper-scissors season has been concluded. Stay tuned for results and rewards.").await?;
    Ok(())
}

async fn startsignupphase_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The signup phase for the new rock-paper-scissors season is now open. Interested players can register.").await?; 
   Ok(())
}

async fn stopsignupphase_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
   bot.send_message(msg.chat.id, "The signup phase is now closed. Preparations for the game will now commence.").await?; 
    Ok(())
}

async fn startgamingphase_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The gaming phase has begun! Players, get ready to challenge each other.").await?;
    Ok(())
}

async fn stopgamingphase_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "The gaming phase has ended. Thank you to all participants!").await?;
    Ok(())
}

async fn approveplayer_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Player has been successfully approved for participation.").await?;
    Ok(())
}

async fn refuseplayer_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Player's request to participate has been refused.").await?;
    Ok(())
}

async fn viewsignuplist_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Here's the list of players who have signed up: ...").await?;
    Ok(())
}

async fn viewapprovedlist_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Here's the list of approved players: ...").await?;
    Ok(())
}

async fn viewrefusedlist_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Here's the list of players whose requests were refused: ...").await?;
    Ok(())
}

async fn viewleaderboard_command(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg) {
        return Ok(());  // Early return if the sender is not authorized
    }
    bot.send_message(msg.chat.id, "Current leaderboard standings: ...").await?;
    Ok(())
}
