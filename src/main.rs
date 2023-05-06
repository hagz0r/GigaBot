use std::env;
use crate::app_settings::SETTINGS;

//use config::{Config, File};
mod app_settings;
mod teloxide_telegram;
mod mongo;

mod models {
    pub mod user;
    pub mod conversation;
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    env::set_var("RUST_LOG", "info");
    env::set_var("TELOXIDE_TOKEN", &*SETTINGS.telegram_bot_token);

    log::info!("Bot started");
    let bot = teloxide::Bot::from_env();

    teloxide_telegram::run(bot).await
} 