// enum.rs

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    //
    //BasicCommands
    //
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Register for a new game season.")]
    Signup,
    #[command(description = "Get the current version.")]
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
    StartNewSeason,
    #[command(description = "off")]
    StopNewSeason,
    #[command(description = "off")]
    StartSignupPhase,
    #[command(description = "off")]
    StopSignupPhase,
    #[command(description = "off")]
    StartGamingPhase,
    #[command(description = "off")]
    StopGamingPhase,
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
    #[command(description = "off")]
    ViewLeaderboard,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These 🌟 Admin 🌟 commands are supported:")]
pub enum AdminCommand {
    #[command(description = "add a user to the admin list.")]
    AddAdmin(String),
    #[command(description = "remove a user from the admin list.")]
    RemoveAdmin(String),
    #[command(description = "list admin users.

        ")]
    ListAdmins,
    #[command(description = "Start a new season for the rock-paper-scissors game.")]
    StartNewSeason,
    #[command(description = "Stop the current season of the rock-paper-scissors game.")]
    StopNewSeason,
    #[command(description = "Begin the signup phase for players.")]
    StartSignupPhase,
    #[command(description = "End the signup phase for players.")]
    StopSignupPhase,
    #[command(description = "Start the gaming phase.")]
    StartGamingPhase,

    #[command(description = "Stop the gaming phase.

        ")]
    StopGamingPhase,
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
    #[command(description = "View the current leaderboard.")]
    ViewLeaderboard,
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

