use mongodb::{bson};
use mongodb::bson::Bson;
use teloxide::types::UserId;
use serde::{Serialize, Deserialize};
use teloxide::prelude::{Message};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConversationBlock {
    pub(crate) user_id: UserId,
    user_request: String,
    bot_response: String,
}

impl ConversationBlock {
    pub fn from_telx(request: &Message, response: &String) -> Self {
        ConversationBlock {
            user_id: request.from().unwrap().id,
            user_request: request.text().unwrap().to_string(),
            bot_response: response.into(),
        }
    }
}

impl From<ConversationBlock> for Bson {
    fn from(block: ConversationBlock) -> Self {
        let doc = bson::to_document(&block).unwrap();
        Bson::Document(doc)
    }
}
