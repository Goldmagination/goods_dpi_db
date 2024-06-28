use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageDTO {
    pub id: i32,
    pub chat_id: i32,
    pub sender_uid: Uuid,
    pub text: String,
    pub timestamp: NaiveDateTime,
    pub is_read: bool,
    pub receiver_uid: Uuid,
    pub assignment: Option<MessageAssignmentDTO>,
}
impl MessageDTO {
    pub fn to_dto(
        id: i32,
        chat_id: i32,
        sender_uid: Uuid,
        text: String,
        timestamp: NaiveDateTime,
        is_read: bool,
        receiver_uid: Uuid,
        assignment: Option<MessageAssignmentDTO>,
    ) -> Self {
        MessageDTO {
            id,
            chat_id,
            sender_uid,
            text,
            timestamp,
            is_read,
            receiver_uid,
            assignment,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageAssignmentDTO {
    pub message_id: i32,
    pub image_url: String,
}
impl MessageAssignmentDTO {
    pub fn to_dto(message_id: i32, image_url: String) -> Self {
        MessageAssignmentDTO {
            message_id,
            image_url,
        }
    }
}
