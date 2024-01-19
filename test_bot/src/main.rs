use teloxide::{
    dispatching::{UpdateHandler},
    prelude::*,
};
use std::{error::Error};
use chrono::{Local, DateTime};
use std::sync::Arc;

mod basic_commands;
mod admin_commands;
mod dev_commands;
mod database;
use database::{init_db_pool, DbPool};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let now: DateTime<Local> = Local::now();
    log::info!("Starting command bot...");
    log::info!("Starting timestamp: {}...", now.format("%Y-%m-%d %H:%M:%S %:z"));
    let db_pool = Arc::new(init_db_pool());
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema(db_pool.clone()))
        .dependencies(dptree::deps![db_pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema(db_pool: Arc<DbPool>) -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<basic_commands::Command, _>()
        .branch(basic_commands::handlers())
        .branch(admin_commands::handlers())
        .branch(dev_commands::handlers(db_pool));

    let message_handler = Update::filter_message()
        .branch(command_handler);
    message_handler
}

