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

