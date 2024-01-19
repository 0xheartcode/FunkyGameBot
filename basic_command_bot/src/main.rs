use teloxide::{
    dispatching::{UpdateHandler},
    prelude::*,
};
use std::{error::Error};
//use dotenv::dotenv;

use chrono::{Local, DateTime};
mod admin;
use admin::{add_admin, remove_admin, list_admins, is_admin};

mod database;
use database::{init_db_pool, DbPool};
use std::sync::Arc;

mod enums;
use enums::{Command};

mod commands;
use commands::basic_commands::{
    help, 
    signup_command, 
    version_command
};

use commands::admin_commands::{
    add_admin_command,
    remove_admin_command,
    list_admins_command,
    startnewseason_command,
    stopnewseason_command,
    startsignupphase_command,
    stopsignupphase_command,
    startgamingphase_command,
    stopgamingphase_command,
    approveplayer_command,
    refuseplayer_command,
    viewsignuplist_command,
    viewapprovedlist_command,
    viewrefusedlist_command,
    viewleaderboard_command,
};

use commands::dev_commands::{
    username_command, 
    username_and_age_command, 
    write_sql_command, 
    read_sql_command
};

use crate::admin::{is_authorized_sender};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let now: DateTime<Local> = Local::now();
    log::info!("Starting command bot...");
    log::info!("Starting timestamp: {}...", now.format("%Y-%m-%d %H:%M:%S %:z"));
    let db_pool = Arc::new(init_db_pool());     
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





