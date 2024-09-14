use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageDTO {
    pub id: i32,
    pub chat_id: i32,
    pub sender_uid: String,
    pub text: Option<String>,
    pub timestamp: NaiveDateTime,
    pub is_read: bool,
    pub receiver_uid: String,
    pub assignments: Option<Vec<MessageAssignmentDTO>>,
}

impl MessageDTO {
    pub fn to_dto(
        id: i32,
        chat_id: i32,
        sender_uid: String,
        text: Option<String>,
        timestamp: NaiveDateTime,
        is_read: bool,
        receiver_uid: String,
        assignments: Option<Vec<MessageAssignmentDTO>>,
    ) -> Self {
        MessageDTO {
            id,
            chat_id,
            sender_uid,
            text,
            timestamp,
            is_read,
            receiver_uid,
            assignments,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
