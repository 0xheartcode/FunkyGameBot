use mongodb::{
    bson::{doc, Document},
    sync::Client,
};
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    // Connect to the MongoDB server
    let client = Client::with_uri_str("mongodb://localhost:27017").expect("Failed to initialize MongoDB client");

    // Access the database and collection
    let db = client.database("mydatabase");
    let collection = db.collection("mycollection");

    // Start Teloxide bot
    let bot = Bot::from_env();

    // Clone the database handle when passing to the closure
    teloxide::repl(bot, |bot, msg| async move {
        answer(bot, msg, db.clone()).await
    })
    .await;
}

async fn answer(bot: Bot, msg: Message, db: mongodb::sync::Database) -> ResponseResult<()> {
    // Log the received message
    log::info!("Received message from: {:?}", msg.chat.username());

    // Insert a document into MongoDB
    let document = doc! { "username": msg.chat.username() };
    collection.insert_one(document, None).expect("Failed to insert document into MongoDB");

    // Send a dice
    bot.send_dice(msg.chat.id).await?;

    // Log that the dice was sent
    log::info!("Sent dice to chat: {:?}", msg.chat.id);

    Ok(())
}

