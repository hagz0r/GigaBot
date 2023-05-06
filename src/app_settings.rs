use lazy_static::lazy_static;
use config::{Config, File};
use serde::Deserialize;

const SETTINGS_FILE_NAME :  &str = "AppSettings";
lazy_static! {
    pub static ref SETTINGS: Settings = {
        let config = Config::builder()
        .add_source(File::new(SETTINGS_FILE_NAME, config::FileFormat::Toml))
        .build()
        .unwrap();
        
        Settings::new(&config)
    };
}

#[derive(Deserialize)]
pub struct Settings {
    pub telegram_bot_token: String,
    pub eth_address: String,
    pub mongo_db: String
}

impl Settings {
    pub fn new(config: &Config) -> Self {
        Settings {
            telegram_bot_token: config.get_string("tg_bot_token").unwrap(),
            eth_address: config.get_string("ETH_Address").unwrap(),
            mongo_db: config.get_string("mongo_DB").unwrap()
        }
    }
}
