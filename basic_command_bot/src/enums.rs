// enum.rs

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    //
    //BasicCommands
    //
    #[command(description = "Display this text. 🟢")]
    Help,
    #[command(description = "Register for a new game season.")]
    Signup,
    #[command(description = "Get the current version. 🟢")]
    Version,
    //
    //DevCommands
    //
    #[command(description = "off")]
    Username,
    #[command(description = "off")]
    UsernameAndAge,
    #[command(description = "off")]
    Writesql(String),
    #[command(description = "off")]
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
    #[command(description = "off")]
    StartNewSeason(String),
    #[command(description = "off")]
    StopNewSeason,
    #[command(description = "off")]
    CurrentSeasonStatus,
    #[command(description = "off")]
    StartSignupPhase,
    #[command(description = "off")]
    StopSignupPhase,
    #[command(description = "off")]
    StartGamingPhase,
    #[command(description = "off")]
    StopGamingPhase,
    #[command(description = "off")]
    StartRound,
    #[command(description = "off")]
    StopRound,    
    #[command(description = "off")]
    ApprovePlayer,
    #[command(description = "off")]
    RefusePlayer,
    #[command(description = "off")]
    ViewSignupList,
    #[command(description = "off")]
    ViewApprovedList,
    #[command(description = "off")]
    ViewRefusedList,
    #[command(description = "View the current leaderboard.")]
    ViewLeaderboard,
    #[command(description = "off")]
    SetBroadcastChannel,
    #[command(description = "off")]
    SetGroupChannel,
    #[command(description = "off")]
    MsgBroadcastChannel,
    #[command(description = "off")]
    MsgGroup,
    #[command(description = "off")]
    GetGroupBroadcastId,
    #[command(description = "off")]
    ResetGroupBroadcast,
    #[command(description = "off")]
    ReadChangelog,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These 🌟 Admin 🌟 commands are supported:")]
pub enum AdminCommand {
    #[command(description = "add a user to the admin list. 🟢")]
    AddAdmin(String),
    #[command(description = "remove a user from the admin list. 🟢")]
    RemoveAdmin(String),
    #[command(description = "list admin users. 🟢 

        ")]
    ListAdmins,
    #[command(description = "Start a new season for the rock-paper-scissors game with a given name and max number of players. 🟢 ")]
    StartNewSeason(String),
    #[command(description = "Stop the current season of the rock-paper-scissors game. 🟢")]
    StopNewSeason,
    #[command(description = "Information regarding the current season. 🟢 

        ")]
    CurrentSeasonStatus,
    #[command(description = "Begin the signup phase for players. 🟢")]
    StartSignupPhase,
    #[command(description = "End the signup phase for players. 🟢")]
    StopSignupPhase,
    #[command(description = "Start the gaming phase. 🟢")]
    StartGamingPhase,

    #[command(description = "Stop the gaming phase. 🟢

        ")]
    StopGamingPhase,
    #[command(description = "The core of the game, start a round.🟠")]
    StartRound,

    #[command(description = "Stop a game round. 🟠

        ")]
    StopRound,    
    #[command(description = "View the list of players who signed up.")]
    ViewSignupList,
    #[command(description = "View the list of approved players.")]
    ViewApprovedList,
    #[command(description = "View the list of refused players.

        ")]
    ViewRefusedList,
    #[command(description = "Approve a player's signup request.")]
    ApprovePlayer,
    #[command(description = "Refuse a player's signup request.

        ")]
    RefusePlayer,
    #[command(description = "Set the channel ID for broadcasting messages. 🟠")]
    SetBroadcastChannel,
    #[command(description = "Set the group channel ID for group-related messages. 🟠")]
    SetGroupChannel,
    #[command(description = "Send a message to the broadcast channel.")]
    MsgBroadcastChannel,
    #[command(description = "Send a message to the group channel.")]
    MsgGroup,
    #[command(description = "Retrieve the current ID of the group and broadcast channel. 🟠")]
    GetGroupBroadcastId,
    #[command(description = "Reset the group and broadcast channel settings.  🟠")]
    ResetGroupBroadcast,
    #[command(description = "Read the changelog. 🟢")]
    ReadChangelog,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These 🤖 Dev 🤖 commands are supported:")]
pub enum DevCommand {
    #[command(description = "displays a username.")]
    Username,
    #[command(description = "basic auth test.")]
    UsernameAndAge,
    #[command(description = "Write to sqllite db.")]
    Writesql(String),
    #[command(description = "Read from sqllite db.")]
    Readsql,
}

