## January 29 Updates

## Updates and New Features

- **Matchmaking Logic on Round Closure**: Implemented a feature for automatic matchmaking when a round is manually closed. Players are now randomly matched against each other.

- **Leaderboard Display**: Added a function to display a leaderboard at the end of each game round.

- **Auto-play for Inactive Players**: Developed a function that automatically plays empty hands for players who are part of the game but haven't played their hand.

- **Player Matching Functionality**: Created a function to match players against each other, which is executed inside the `/stopround` function.

- **Scoring System**: Integrated a scoring system where players earn points based on the outcome of their matches.

- **Final Leaderboard Announcement**: Implemented a feature to display a special final leaderboard at the conclusion of a season.

## Bug Fixes and Improvements

- Fixed various bugs related to database interactions and command logic.
- Improved the clarity and user-friendliness of bot messages and responses.
- Streamlined the code for better performance and maintainability.

## January 28 Updates

### Command Implementations and Modifications

- `signup_command`: Implemented to allow players to sign up for the current game season. The command checks if the game is in the "start_signup" phase and registers the player in the `MasterCandidateTable` with 'pending' status.

- `view_signuplist_command`: Enhanced to view the list of players who have signed up for the current season. The command now supports filtering by player status ('pending', 'accepted', 'refused', or 'all').

- `refuseplayer_command`: Updated to set a player's status to 'refused' in the `MasterCandidateTable`. It checks if the player is pending in the current season before updating.

- `approveplayer_command`: Adapted to not only update the player's status to 'accepted' in the `MasterCandidateTable`, but also to add the player to the `PlayerDetailsTable`. Now returns a message to the accepted player.

- `playrock_command`, `playpaper_command`, `playscissors_command`: Created to allow players to play their respective hands in the ongoing round. These commands check if the player is part of the current game and inserts their choice into the `RoundDetailsTable`.

### Database Function Enhancements

- `get_signup_list_for_season`: Adjusted to filter the signup list based on player status. Added functionality to handle different status filters including 'all'.

- `insert_player_hand_choice`: Created as a database utility function to insert a player's game choice into `RoundDetailsTable`. Returns a boolean indicating the success of the operation.

### Database Reset

- Performed a reset of the database to ensure a fresh start with the newly implemented functionalities and schema adjustments.

## January 26 Updates

### Function Implementations and Bug Fixes

- Continued implementing and finished implementing the round-related functions, starting and stopping game rounds.

- Added a new 'round_ongoing' status.
-
## January 24 Updates

- database_schema.md published

- started implementing the round function. Logic not there yet.

## January 23 Updates

### New Commands

- `/setbroadcastchannel`: Sets the ID for the broadcast channel.

- `/setgroupchannel`: Sets the ID for the group channel.

- `/getgroupbroadcastid`: Retrieves the current IDs of the group and broadcast channels.

- `/resetgroupbroadcast`: Resets the settings for the group and broadcast channels.

### Database Schema Update

- Modified the database schema to include a new table `channel_settings`. This table is designed to store the IDs for the broadcast and group channels.


# January 22, 2024
## New Command Implementations:

- StartSignupPhase: Initiates the signup phase for the rock-paper-scissors season, allowing player registrations.

- StopSignupPhase: Ends the signup phase, transitioning towards the gaming phase.

- StartGamingPhase: Begins the gaming phase of the season, marking the start of the game.

- StopGamingPhase: Concludes the gaming phase, leading towards the season's closure.

## Enhancements:

- Updated the seasons table schema to include a status column for better phase management.

- Enhanced command logic to allow starting the signup phase from either 'initial' or 'stopped_signup' status.




# January 21, 2024

## Command Structuring and Parsing:

- Implemented parsing of complex command inputs (e.g., `/command String Int`). Better to pipe to String and then let the command handle it directly.

## Database Enhancements:

- Added functions to check for an active season, season details, start a new season, and stop the current season.

## Error Handling Improvements:

- Addressed various error handling issues, particularly related to database connections and command execution.
- Improved error messages for better debugging and user feedback.

## File Reading for Changelog:

- Created functionality to read from a `changelog.txt` file and send its contents as a message in Telegram.

