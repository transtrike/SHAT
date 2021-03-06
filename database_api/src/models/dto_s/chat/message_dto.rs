use serde::{Deserialize, Serialize};

use crate::models::pure::chat::message::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDTO {
    pub id: i32,
    pub issuer_id: i32,
    pub issuer: String,
    pub message: String,
}

impl From<Message> for MessageDTO {
    fn from(_message: Message) -> Self {
        todo!()
    }
}
