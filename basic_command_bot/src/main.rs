/// main.rs

use teloxide::{
    dispatching::{UpdateHandler},
    prelude::*,
};

//use teloxide::adaptors::throttle::Limits;

//use dotenv::dotenv;

use chrono::{Local, DateTime};
mod admin;


mod database;
use database::{init_db_pool, DbPool};
use std::sync::Arc;

mod enums;
use enums::{Command};

mod commands;
use commands::basic_commands::{
    help, 
    signup_command, 
    version_command,
    viewleaderboard_command,
    status_command,
    playrock_command,
    playpaper_command,
    playscissors_command,
};

use commands::admin_commands::{
    add_admin_command,
    remove_admin_command,
    list_admins_command,
    start_new_season_command,
    stop_new_season_command,
    current_season_status_command,
    startsignupphase_command,
    stopsignupphase_command,
    startgamingphase_command,
    stopgamingphase_command,
    start_round_command,
    stop_round_command,
};

use commands::registration_commands::{
    approveplayer_command,
    refuseplayer_command,
    view_signuplist_command,
    view_approved_list_command,
    viewrefusedlist_command,
};

use commands::dev_commands::{
    username_command, 
    username_and_age_command, 
    write_sql_command, 
    read_sql_command,
    admin_reset_players_command,
    admin_reset_candidate_command,
};

use commands::grp_broadcast_commands::{
    set_broadcast_channel_command,
    set_group_channel_command,
    get_group_broadcast_id_command,
    reset_group_broadcast_command,
    msg_broadcastchannel_command,
    msg_group_command,
};

use commands::changelogread::{
    send_changelog,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let now: DateTime<Local> = Local::now();
    log::info!("Starting command bot...");
    log::info!("Starting timestamp: {}...", now.format("%Y-%m-%d %H:%M:%S %:z"));
    let db_pool = Arc::new(init_db_pool());     
    let bot = Bot::from_env()
        //.throttle(Limits::default())
        ;

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
        .branch(
            case![Command::Help].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    help(bot, msg, db_pool).await
                }
            )
        )
        .branch(
            case![Command::Signup].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    signup_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(case![Command::Version].endpoint(version_command))
        .branch(
            case![Command::ViewLeaderboard].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    viewleaderboard_command(bot, msg, &db_pool).await
                }
            )
        ) 
        .branch(
            case![Command::Status].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    status_command(bot, msg, &db_pool).await
                }
            )
        ) 
        .branch(
            case![Command::PlayRock].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    playrock_command(bot, msg, &db_pool).await
                }
            )
        ) 
        .branch(
            case![Command::PlayPaper].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    playpaper_command(bot, msg, &db_pool).await
                }
            )
        ) 
        .branch(
            case![Command::PlayScissors].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    playscissors_command(bot, msg, &db_pool).await
                }
            )
        ) 
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
    .branch(dptree::case![Command::ResetPlayerTable].endpoint(
            |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                admin_reset_players_command(bot, msg, db_pool).await
            }))
    .branch(dptree::case![Command::ResetCandidateTable].endpoint(
            |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                admin_reset_candidate_command(bot, msg, db_pool).await
            }))
    //
    //AdminCommands
    //
    .branch(
        case![Command::StartNewSeason(season_info)].endpoint(
            |bot: Bot, msg: Message, db_pool: Arc<DbPool>, season_info: String| async move {
                start_new_season_command(bot, msg, &db_pool, season_info).await
            }
        )
    )
        .branch(
            case![Command::StopNewSeason].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    stop_new_season_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::CurrentSeasonStatus].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    current_season_status_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::StartSignupPhase].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    startsignupphase_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::StopSignupPhase].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    stopsignupphase_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::StartGamingPhase].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    startgamingphase_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::StopGamingPhase].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    stopgamingphase_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::StartRound].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    start_round_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::StopRound].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    stop_round_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::ApprovePlayer(username)].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>, username: String| async move {
                    approveplayer_command(bot, msg, &db_pool, username).await
                }
            )
        )
        .branch(
            case![Command::RefusePlayer(username)].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>, username: String| async move {
                    refuseplayer_command(bot, msg, &db_pool, username).await
                }
            )
        )
        .branch(
            case![Command::ViewSignupList].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    view_signuplist_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::ViewApprovedList].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    view_approved_list_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::ViewRefusedList].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    viewrefusedlist_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::SetBroadcastChannel(channel_id)].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>, channel_id: String| async move {
                    set_broadcast_channel_command(bot, msg, &db_pool, channel_id).await
                }
            )
        )
        .branch(
            case![Command::SetGroupChannel(channel_id)].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>, channel_id:String| async move {
                    set_group_channel_command(bot, msg, &db_pool, channel_id).await
                }
            )
        )
        .branch(
            case![Command::MsgBroadcastChannel(message_text)].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>, message_text:String| async move {
                    msg_broadcastchannel_command(bot, msg, &db_pool, message_text).await
                }
            )
        )
        .branch(
            case![Command::MsgGroup(message_text)].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>, message_text:String| async move {
                    msg_group_command(bot, msg, &db_pool, message_text).await
                }
            )
        )
        .branch(
            case![Command::GetGroupBroadcastId].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    get_group_broadcast_id_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::ResetGroupBroadcast].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    reset_group_broadcast_command(bot, msg, &db_pool).await
                }
            )
        )
        .branch(
            case![Command::ReadChangelog].endpoint(
                |bot: Bot, msg: Message| async move {
                    send_changelog(bot, msg).await
                }
            )
        )
        .branch(
            case![Command::ListAdmins].endpoint(
                |bot: Bot, msg: Message, db_pool: Arc<DbPool>| async move {
                    list_admins_command(bot, msg, &db_pool).await
                }
            )
        )

        .branch(
            dptree::case![Command::AddAdmin(username)]
            .endpoint(|bot: Bot, msg: Message, db_pool: Arc<DbPool>, username: String| async move {
                add_admin_command(bot, msg, &db_pool, username).await
            })
        )
        .branch(
            dptree::case![Command::RemoveAdmin(username)]
            .endpoint(|bot: Bot, msg: Message, db_pool: Arc<DbPool>, username: String| async move {
                remove_admin_command(bot, msg, &db_pool, username).await
            })
        );

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(handle_invalid_text_message)) 
        ;
    message_handler

}

// When you don't receive a message that is a command (starts with /)
async fn handle_invalid_text_message(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(username) = msg.from().and_then(|user| user.username.clone()) {
        log::info!("ChatId: {}, Date {} \nFrom: {} Received an invalid text message. \nContent: {}", msg.chat.id, msg.date, username, msg.text().unwrap_or_default());
        //log::info!("{:?}",msg);
    }
    bot.send_message(msg.chat.id, "Received your message, this is not a valid command. Try /help.").await?;
    Ok(())
}



