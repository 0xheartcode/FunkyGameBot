use std::fs;
use std::error::Error;
use std::path::Path;
use std::env;
use std::path::PathBuf;
use teloxide::{prelude::*, utils::command::BotCommands};

const MAX_MESSAGE_LENGTH: usize = 4096;

pub async fn send_changelog(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut changelog_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    changelog_path.push("./Changelog.txt");
    let changelog = fs::read_to_string(changelog_path)?;

    // Define the characters that need to be escaped
    let characters_to_escape = "_*[]()~`>#+-=|{}.!";

    // Function to escape characters in a string
    fn escape_characters(input: &str, characters: &str) -> String {
        let mut escaped_string = String::new();
        for c in input.chars() {
            if characters.contains(c) {
                escaped_string.push('\\'); // Add the escape character
            }
            escaped_string.push(c);
        }
        escaped_string
    }

    // Split the changelog into chunks of MAX_MESSAGE_LENGTH
    for chunk in changelog.as_bytes().chunks(MAX_MESSAGE_LENGTH) {
        let message = String::from_utf8_lossy(chunk);
        let escaped_message = escape_characters(&message, characters_to_escape);
        bot.send_message(msg.chat.id, escaped_message.to_string())
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
    }

    Ok(())
}
 
