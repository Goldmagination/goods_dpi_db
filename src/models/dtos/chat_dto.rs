use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};

use crate::models::
{
    chat_aggregate::chat::*,
};

#[derive(Debug, Serialize, Deserialize, )]
pub struct ChatDTO {
    pub id: i32,
    pub professional_profile_name: String,
    pub last_message: String,
    pub professional_profile_image_url: Option<String>,
    pub time: NaiveDateTime,
}

impl ChatDTO {
    pub fn chat_to_dto(chat: &Chat, professional_profile_name: String, last_message:String, image_url:Option<String>) -> ChatDTO {
        ChatDTO {
            id: chat.id,
            professional_profile_name: professional_profile_name.clone(),
            last_message:last_message.clone(),
            time:chat.last_message_time,
            professional_profile_image_url: image_url.clone()
        }
    }
}
