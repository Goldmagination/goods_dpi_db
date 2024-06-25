use crate::schema::schema::chat;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize, Identifiable)]
#[diesel(table_name = chat)]
pub struct Chat {
    pub id: i32,
    pub user_id: i32,
    pub professional_profile_id: i32,
    pub last_message_time: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = chat)]
pub struct NewChat {
    pub user_id: i32,
    pub professional_profile_id: i32,
    pub last_message_time: NaiveDateTime,
}
pub struct ChatDTO {
    pub id: i32,
    pub user_id: i32,
    pub professional_profile_id: i32,
    pub professional_name: String,
    pub image_url: Option<String>,
    pub last_message: String,
}
