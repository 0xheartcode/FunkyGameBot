/// admin_commands.rs

use teloxide::{prelude::*};
use std::{error::Error, sync::Arc};

use crate::admin::{
    is_authorized_sender, 
    list_admins, 
    is_admin, 
    remove_admin, 
    add_admin
};

use crate::database::{
    DbPool
};

use crate::commands::season::{
    start_new_season, 
    stop_current_season, 
    current_active_season, 
    current_active_season_id, 
    current_active_season_details,
    start_signup_phase,
    stop_signup_phase,
    start_gaming_phase,
    stop_gaming_phase,
    get_next_round_number,
    start_new_round,
    end_current_round,
    play_empty_hands_for_players,
};

use crate::commands::playing_commands::{
     get_current_round_id,
     get_player_hands,
     random_match_players,
     evaluate_matches,
     update_player_score,
     announce_results,
};

use crate::commands::playing_commands::fetch_leaderboard;
 use crate::commands::playing_commands::prepare_leaderboard_string;


//
//TODO AdminCommands
//
//====================================================


pub async fn add_admin_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    let username = username.trim();
    if username.is_empty() {
        bot.send_message(msg.chat.id, "Please provide a non-empty username.").await?;
    } else if username.split_whitespace().count() != 1 {
        bot.send_message(msg.chat.id, "Only one username please, no spaces.").await?;
    } else if is_admin(db_pool, &username)? {
        bot.send_message(msg.chat.id, format!("@{} is already an admin.", username)).await?;
    } else {
        //ignoring for now. Might handle Err properly later.
        let _ = add_admin(db_pool, username);
        bot.send_message(msg.chat.id, format!("Added @{} to admin list.", username)).await?;
    }
    Ok(())
}

pub async fn remove_admin_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    let username = username.trim();
    if username.is_empty() || !is_admin(db_pool, &username).unwrap_or(false) {
        if username.is_empty() {
            bot.send_message(msg.chat.id, "Your command is empty, we need 1 username here.").await?;
        } else {
            bot.send_message(msg.chat.id, format!("User @{} is not in the admin list.", username)).await?;
        }
    } else {
        //ignoring for now. Might handle Err properly later.
        let _ = remove_admin(db_pool, &username); 
        bot.send_message(msg.chat.id, format!("Removed @{} from admin list.", username)).await?;
    }
    Ok(())
}

pub async fn list_admins_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    let admins = list_admins(db_pool)?;
    let mut response = String::from("Admins:\n");
    for admin in admins {
    response.push_str(&format!("@{}\n", admin));
    }

    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}



pub async fn start_new_season_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>, season_info: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    
    let season_info = season_info.trim();
    if season_info.split_whitespace().count() != 2 {
        bot.send_message(msg.chat.id, "The command should be used like this '/startnewseason <Title> <Number of Players>'.").await?;
        return Ok(());
    }

    let mut parts = season_info.split_whitespace();

    // Get the first part as the season_name
    let season_name = match parts.next() {
        Some(name) => name.to_string(),  // Convert the &str slice to a String
        None => {
            bot.send_message(msg.chat.id, "Please provide a season name.").await?;
            return Ok(());
        }
    };

    // Get the last part as max_players
    let max_players: i32 = match parts.next() {  // Changed from parts.last() to parts.next()
        Some(num_str) => match num_str.parse() {
            Ok(num) => num,
            Err(_) => {
                bot.send_message(msg.chat.id, "Invalid number format for max players.").await?;
                return Ok(());
            }
        },
        None => {
            bot.send_message(msg.chat.id, "Please provide the maximum number of players.").await?;
            return Ok(());
        }
    };
       // Check if there is an active season
    match current_active_season(db_pool).await {
        Ok(Some(active_season)) => {
            // There is an active season, send a message
            bot.send_message(msg.chat.id, format!("A season is already in progress: '{}'. Another season cannot be started until the current one is concluded.", active_season)).await?;
        },
        Ok(None) => {
            // No active season, proceed to start a new one
            match start_new_season(db_pool, &season_name, max_players).await {
                Ok(_) => {
                    // Successfully started a new season
                    bot.send_message(msg.chat.id, format!("A new rock-paper-scissors season '{}' has started! Maximum players allowed: {}. Let the games begin.", season_name, max_players)).await?;
                },
                Err(e) => {
                    // Error in starting a new season
                    bot.send_message(msg.chat.id, format!("Failed to start new season '{}': {}", season_name, e)).await?;
                }
            }
        },
        Err(e) => {
            // Error in checking for an active season
            bot.send_message(msg.chat.id, format!("Error checking for active season: {}", e)).await?;
        }
    }

    Ok(())
}

pub async fn stop_new_season_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    // Check if there is an active season
    match current_active_season_details(db_pool).await {
        Ok(Some((season_id,season_name, _, _, _))) => {
            // Stop the active season
            match stop_current_season(db_pool).await {
                Ok(_) => {
                    // Successfully stopped the season
                    bot.send_message(msg.chat.id, format!("The season '{}' has been successfully concluded.", season_name)).await?;

                   // Fetch and display the final leaderboard
                    match fetch_leaderboard(db_pool, season_id).await {
                        Ok(leaderboard) => {
                            let mut response = prepare_leaderboard_string(leaderboard).await;
                            // Replace the first line with the final leaderboard title
                            if let Some(end_of_first_line) = response.find('\n') {
                                response.replace_range(..end_of_first_line, &format!("🏆 Final {} Leaderboard 🏆", season_name));
                            }
                            bot.send_message(msg.chat.id, &response).await?;
                        },
                        Err(_) => {
                            bot.send_message(msg.chat.id, "Failed to fetch the final leaderboard.").await?;
                        }
                    }
                }
                Err(e) => {
                    // Error in stopping the season
                    bot.send_message(msg.chat.id, format!("Failed to conclude the season '{}': {}", season_name, e)).await?;
                }
            }
        }
        Ok(None) => {
            // No active season
            bot.send_message(msg.chat.id, "There is no active season to conclude.").await?;
        }
        Err(e) => {
            // Error in getting season details
            bot.send_message(msg.chat.id, format!("Failed to get details of the current season: {}", e)).await?;
        }
    }

    Ok(())

}


pub async fn current_season_status_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Check if there is an active season
    match current_active_season_details(db_pool).await {
        Ok(Some((_, name, start_date, max_players, status))) => {
            let message = format!(
                "Current active season: '{}'\nStarted on: {}\nMax players: {}\nStatus: {}",
                name, start_date, max_players, status
            );
            bot.send_message(msg.chat.id, message).await?;
        },
        Ok(None) => {
            bot.send_message(msg.chat.id, "There is no active season currently.").await?;
        },
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to get current season status: {}", e)).await?;
        }
    }

    Ok(())
}




pub async fn startsignupphase_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Attempt to start the signup phase
    // Check the current season's status
    match current_active_season_details(db_pool).await {
        Ok(Some((_, name, _, _, status))) => {
            match status.as_str() {
                "start_signup" => {
                    bot.send_message(msg.chat.id, "Signup has already started.").await?;
                },
                "round_ongoing" => {
                    bot.send_message(msg.chat.id, "A round is going on, you cannot start signing up.").await?;
                },
                "start_gaming" => {
                    bot.send_message(msg.chat.id, "The game has already started. We cannot open the signup now. Let's be fair.").await?;
                },
                "stopped_gaming" => {
                    bot.send_message(msg.chat.id, "The game already started. And seems it also ended. Not a time to open sign-ups.").await?;
                },
                _ => {
                    // If none of the above, attempt to start the signup phase
                    match start_signup_phase(db_pool).await {
                        Ok(_) => {
                            bot.send_message(msg.chat.id, format!("The signup phase for the new rock-paper-scissors season '{}' is now open. Interested players can register.", name)).await?;
                        },
                        Err(e) => {
                            log::info!("Failed to start the signup phase: {}", e);
                            bot.send_message(msg.chat.id, format!("Failed to start the signup phase: {}", e)).await?;
                        }
                    }
                }
            }
        },
        Ok(None) => {
            bot.send_message(msg.chat.id, "There is no active season currently.").await?;
        },
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to get current season status: {}", e)).await?;
        }
    }

    Ok(())
}

pub async fn stopsignupphase_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Check the current season's status
    match current_active_season_details(db_pool).await {
        Ok(Some((_, name, _, _, status))) => {
            match status.as_str() {
                "stopped_signup" => {
                    bot.send_message(msg.chat.id, "Signup has already stopped!").await?;
                },
                "round_ongoing" => {
                    bot.send_message(msg.chat.id, "A round is going on, you cannot stop signing up.").await?;
                },
                "start_gaming" => {
                    bot.send_message(msg.chat.id, "The game has already started. This command is not valid.").await?;
                },
                "stopped_gaming" => {
                    bot.send_message(msg.chat.id, "The game already started. And seems it also ended. This command is not valid.").await?;
                },
                _ => {
                    // If none of the above, attempt to start the signup phase
                    match stop_signup_phase(db_pool).await {
                        Ok(_) => {
                            bot.send_message(msg.chat.id, format!("The signup phase is now closed. Preparations for the '{}' game will now commence.", name)).await?;
                        },
                        Err(e) => {
                            log::info!("Failed to stop the signup phase: {}", e);
                            bot.send_message(msg.chat.id, format!("Failed to stop the signup phase: {}", e)).await?;
                        }
                    }
                }
            }
        },
        Ok(None) => {
            bot.send_message(msg.chat.id, "There is no active season currently.").await?;
        },
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to get current season status: {}", e)).await?;
        }
    }

    Ok(())
}


pub async fn startgamingphase_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    // Check the current season's status
    match current_active_season_details(db_pool).await {
        Ok(Some((_, name, _, _, status))) => {
            match status.as_str() {
                "start_gaming" => {
                    bot.send_message(msg.chat.id, "The game has already started!").await?;
                },
                "round_ongoing" => {
                    bot.send_message(msg.chat.id, "A round is going on, you cannot start gaming phase.").await?;
                },
                "start_signup" => {
                    bot.send_message(msg.chat.id, "The signup phase has not been completed. Please finish it first.").await?;
                },
                "initial" => {
                    bot.send_message(msg.chat.id, "The season just started. Please start the signup phase first, we need players.").await?;
                },
                _ => {
                    // If none of the above, attempt to start the signup phase
                    match start_gaming_phase(db_pool).await {
                        Ok(_) => {
                            bot.send_message(msg.chat.id, format!("The gaming phase has begun! Welcome to '{}'.Players, get ready to challenge each other.", name)).await?;
                        },
                        Err(e) => {
                            log::info!("Failed to start the signup phase: {}", e);
                            bot.send_message(msg.chat.id, format!("Failed to start the signup phase: {}", e)).await?;
                        }
                    }
                }
            }
        },
        Ok(None) => {
            bot.send_message(msg.chat.id, "There is no active season currently.").await?;
        },
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to get current season status: {}", e)).await?;
        }
    }

    Ok(())
}

pub async fn stopgamingphase_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }
    // Check the current season's status
    match current_active_season_details(db_pool).await {
        Ok(Some((_, _, _, _, status))) => {
            match status.as_str() {
                "stopped_gaming" => {
                    bot.send_message(msg.chat.id, "The game has already started!").await?;
                },
                "round_ongoing" => {
                    bot.send_message(msg.chat.id, "A round is going on, you cannot stop gaming phase.").await?;
                },
                "stopped_signup" => {
                    bot.send_message(msg.chat.id, "Oh, the signup is closed, however, the game hasn't started yet. Start a game to close it.").await?;
                },
                "start_signup" => {
                    bot.send_message(msg.chat.id, "The signup phase has not been completed. Please finish it first, and then start the gaming phase.").await?;
                },
                "initial" => {
                    bot.send_message(msg.chat.id, "The season just started. Please start the signup phase first, stop it, start the game phase. Then we can talk about closing the game.").await?;
                },
                _ => {
                    // If none of the above, attempt to start the signup phase
                    match stop_gaming_phase(db_pool).await {
                        Ok(_) => {
                            bot.send_message(msg.chat.id, format!("The gaming phase has ended. Thank you to all participants! Remember to /stopnewseason when you're done.")).await?;
                        },
                        Err(e) => {
                            log::info!("Failed to stop the gaming phase: {}", e);
                            bot.send_message(msg.chat.id, format!("Failed to stop the gaming phase: {}", e)).await?;
                        }
                    }
                }
            }
        },
        Ok(None) => {
            bot.send_message(msg.chat.id, "There is no active season currently.").await?;
        },
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to get current season status: {}", e)).await?;
        }
    }    

    Ok(())
}

pub async fn start_round_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Check the current season's status
    let season_details = current_active_season_details(db_pool).await?;
    if let Some((_, _, _, _, status)) = season_details {
        if status != "start_gaming" {
            bot.send_message(msg.chat.id, "There is no active season in the 'start_gaming' phase.").await?;
            return Ok(());
        }
    } else {
        bot.send_message(msg.chat.id, "No active season found.").await?;
        return Ok(());
    }

    // Get the current active season ID
    let current_season_id = current_active_season_id(db_pool).await?;
    if let Some(season_id_str) = current_season_id {
        // Get the next round number
        let next_round_number = get_next_round_number(db_pool, &season_id_str).await?;

        // Start the new round
        start_new_round(db_pool, season_id_str, next_round_number).await?;
        bot.send_message(msg.chat.id, "Finally, let us start the round!").await?;
    } else {
        bot.send_message(msg.chat.id, "No active season ID found.").await?;
    }

    Ok(())
}

pub async fn stop_round_command(bot: Bot, msg: Message, db_pool: &Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !is_authorized_sender(&msg, db_pool) {
        return Ok(());  // Early return if the sender is not authorized
    }

    // Check the current season's status
    let season_details = current_active_season_details(db_pool).await?;
    if season_details.is_none() || season_details.unwrap().4 != "round_ongoing" {
        bot.send_message(msg.chat.id, "There is no active season in the 'round_ongoing' phase.").await?;
        return Ok(());
    }

    // Get the current active round's ID
    let current_season_id = current_active_season_id(db_pool).await?;
    if let Some(season_id) = current_season_id {
        
        // Get the current round ID
        let current_round_id = get_current_round_id(db_pool, season_id)
            .await?
            .expect("No current round ID found");
        
        // Handle players who haven't played
        let players_without_moves = play_empty_hands_for_players(db_pool, season_id, current_round_id).await?;

        // Notify players who haven't played
        for player_id in players_without_moves {
            // Here, send a message to each player. You'll need to convert `player_id` to a ChatId or UserId that Bot can use
            let chat_id = ChatId(player_id as i64); // Assuming player_id can be directly used as chat_id
            bot.send_message(chat_id, "Your hand was empty for this round.").await?;
        }

        // ====
        // ==== GAME MAKING LOGIC HERE ===
        // ====
        // Step 1: Retrieve players and hands
        let player_hands = get_player_hands(db_pool, current_round_id, season_id).await?;
        println!("Player hands: {:?}",player_hands);

        // Step 2: Randomly match players
        let matched_pairs = random_match_players(player_hands).await;
        println!("Matched pairs: {:?}",matched_pairs);

        // Step 3: Evaluate matches
        let match_results = evaluate_matches(db_pool, matched_pairs, current_round_id).await?;
        println!("Match results: {:?}", match_results);

        // Step 4: Update player scores and RoundDetailsTable
        update_player_score(db_pool, match_results.clone(), season_id).await?;

        // End of game-making logic

        // End the current round
        end_current_round(db_pool, season_id).await?;

        bot.send_message(msg.chat.id, "Round is over, everyone back to their corner!").await?;

        // post game announcements:
        // Generate announcement for match results
        let announcement = announce_results(db_pool, match_results).await?;
        if !announcement.is_empty() {
            bot.send_message(msg.chat.id, &announcement).await?;
        }



    } else {
        bot.send_message(msg.chat.id, "No active season ID found.").await?;
    }

    Ok(())
}



