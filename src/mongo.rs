use std::error::Error;
use std::fmt;
use mongodb::{Client, Collection, Database, options::{ClientOptions, ResolverConfig}};
use lazy_static::lazy_static;
use mongodb::bson::{doc};
use crate::{
    models::user::AppUser,
    app_settings::SETTINGS,
    models::conversation::ConversationBlock,
};
use async_once::AsyncOnce;
use mongodb::options::UpdateOptions;
use teloxide::prelude::UserId;



lazy_static! {
    pub static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
        let uri = &*SETTINGS.mongo_db;
        let options =
        ClientOptions::parse_with_resolver_config(&uri, ResolverConfig::cloudflare())
        .await.unwrap();
       
        
        Client::with_options(options).unwrap()
    });
}


#[derive(Debug)]
pub enum AppDBError {
    /* idk if gigachat api will limit conversation.
        so i put it for next updates
     */
    #[warn(dead_code)]
    MessageLimitExceeded,
}

impl fmt::Display for AppDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppDBError::MessageLimitExceeded => write!(f, "Message limit exceeded"),
        }
    }
}

impl Error for AppDBError {}


pub async fn get_database(db_name: &str) -> Database {
    CLIENT.get().await.database(db_name)
}

pub fn get_collection<T>(collection: &str, db: &Database) -> Collection<T> {
    db.collection(collection)
}


pub async fn clear_conversation(user_id: UserId, collection: &Collection<AppUser>) {
    let filter = doc! {
        "telx_user.id" : user_id.0 as i64,
    };

    let update = doc! {
        "$set" : {"conversation" : [] }
    };
    collection.update_one(filter, update, None).await.unwrap();
}

pub async fn get_conversation(user_id: UserId, collection: &Collection<AppUser>) -> Result<Vec<ConversationBlock>, Box<dyn Error>> {
    let filter = doc! { "telx_user.id": user_id.0 as i64 };
    let result = collection.find_one(filter, None).await?;

    match result {
        Some(user) => Ok(user.conversation),
        None => Ok(Vec::new())
    }
}


pub async fn add_message(collection: &Collection<AppUser>, conversation_block: &ConversationBlock) -> Result<(), Box<dyn Error>> {
    let filter = doc! { "telx_user.id": conversation_block.user_id.0 as i64 };
    let update = doc! { "$push": { "conversation": conversation_block } };

    collection.update_one(filter, update, None).await?;

    Ok(())
}

pub async fn upsert_user(app_user: &AppUser, collection: &Collection<AppUser>) -> Result<(), Box<dyn Error>> {
    let filter = doc! { "telx_user.id": app_user.telx_user.id.0 as i64 };
    let update = doc! { "$set": mongodb::bson::to_bson(app_user)? };

    collection.update_one(filter, update, UpdateOptions::builder().upsert(true).build()).await?;

    Ok(())
}

