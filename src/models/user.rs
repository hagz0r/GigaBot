use teloxide::types::User;
use serde::{Serialize, Deserialize};
use crate::models::conversation::ConversationBlock;


#[derive(Serialize, Deserialize, Clone)]
pub struct AppUser {
    pub telx_user: User,
    pub conversation: Vec<ConversationBlock>,
}


impl AppUser {
    pub fn new(telx_user: User, conversation: Vec<ConversationBlock>) -> Self {
        AppUser {
            telx_user,
            conversation,
        }
    }
}



