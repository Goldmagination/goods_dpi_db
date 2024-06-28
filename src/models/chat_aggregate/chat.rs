use crate::models::dtos::message_dto::{MessageAssignmentDTO, MessageDTO};
use crate::schema::schema::chat;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = chat)]
pub struct Chat {
    pub id: i32,
    pub last_message_time: NaiveDateTime,
    pub user_uid: Uuid,
    pub professional_profile_uid: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = chat)]
pub struct NewChat {
    pub user_uid: Uuid,
    pub professional_profile_uid: Uuid,
    pub last_message_time: NaiveDateTime,
}

#[derive(Serialize)]
pub struct ChatDTO {
    pub id: i32,
    pub user_id: Uuid,
    pub professional_profile_id: Uuid,
    pub professional_name: String,
    pub image_url: Option<String>,
    pub messages: Option<Vec<MessageDTO>>,
}
