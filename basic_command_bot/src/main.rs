use teloxide::{
    dispatching::{UpdateHandler},
    prelude::*,
};

//use dotenv::dotenv;

use chrono::{Local, DateTime};
mod admin;


mod database;
use database::{init_db_pool, DbPool};
use std::sync::Arc;

mod enums;
use enums::{Command, DevCommand, AdminCommand};

mod commands;
use commands::basic_commands::{
    help, 
    signup_command, 
    version_command,
    viewleaderboard_command,
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
};

use commands::dev_commands::{
    username_command, 
    username_and_age_command, 
    write_sql_command, 
    read_sql_command
};



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
        .branch(case![Command::ViewLeaderboard].endpoint(viewleaderboard_command))
        //
        //DevCommands
        //
        .branch(case![DevCommand::Username].endpoint(username_command))
        .branch(case![DevCommand::UsernameAndAge].endpoint(username_and_age_command))
        .branch(dptree::case![DevCommand::Writesql(value)].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>, value: String| async move {
                write_sql_command(bot, msg, db_pool, value).await
        }))
        .branch(dptree::case![DevCommand::Readsql].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
            read_sql_command(bot, msg, db_pool).await
        }))
        //
        //AdminCommands
        //
        .branch(case![AdminCommand::StartNewSeason].endpoint(startnewseason_command))
        .branch(case![AdminCommand::StopNewSeason].endpoint(stopnewseason_command))
        .branch(case![AdminCommand::StartSignupPhase].endpoint(startsignupphase_command))
        .branch(case![AdminCommand::StopSignupPhase].endpoint(stopsignupphase_command))
        .branch(case![AdminCommand::StartGamingPhase].endpoint(startgamingphase_command))
        .branch(case![AdminCommand::StopGamingPhase].endpoint(stopgamingphase_command))
        .branch(case![AdminCommand::ApprovePlayer].endpoint(approveplayer_command))
        .branch(case![AdminCommand::RefusePlayer].endpoint(refuseplayer_command))
        .branch(case![AdminCommand::ViewSignupList].endpoint(viewsignuplist_command))
        .branch(case![AdminCommand::ViewApprovedList].endpoint(viewapprovedlist_command))
        .branch(case![AdminCommand::ViewRefusedList].endpoint(viewrefusedlist_command))
        .branch(case![AdminCommand::ListAdmins].endpoint(list_admins_command))
        .branch(
            dptree::case![AdminCommand::AddAdmin(username)]
            .endpoint(|bot: Bot, msg: Message, username: String| async move {
                add_admin_command(bot, msg, username).await
            })
        )
        .branch(
            dptree::case![AdminCommand::RemoveAdmin(username)]
            .endpoint(|bot: Bot, msg: Message, username: String| async move {
                remove_admin_command(bot, msg, username).await
            })
        );

    let message_handler = Update::filter_message()
        .branch(command_handler);
    message_handler

}





