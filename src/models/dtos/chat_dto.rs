use crate::models::chat_aggregate::chat::*;
use crate::models::dtos::message_dto::MessageDTO;
use serde::Serialize;

#[derive(Serialize)]
pub struct ChatDTO {
    pub id: i32,
    pub user_id: String,
    pub professional_profile_id: String,
    pub professional_name: String,
    pub image_url: Option<String>,
    pub messages: Option<Vec<MessageDTO>>,
}

impl ChatDTO {
    pub fn chat_to_dto(
        chat: Chat,
        professional_name: String,
        image_url: Option<String>,
        messages: Option<Vec<MessageDTO>>,
    ) -> ChatDTO {
        ChatDTO {
            id: chat.id,
            user_id: chat.user_uid,
            professional_profile_id: chat.professional_profile_uid,
            professional_name,
            image_url,
            messages,
        }
    }
}
