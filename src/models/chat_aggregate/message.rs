use crate::schema::schema::message;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub text: String,
    pub timestamp: NaiveDateTime,
    pub is_read: bool,
    pub receiver_uid: String,
    pub sender_uid: String,
}

#[derive(Insertable)]
#[diesel(table_name = message)]
pub struct NewMessage {
    pub chat_id: i32,
    pub sender_uid: String,
    pub receiver_uid: String,
    pub text: String,
    pub timestamp: NaiveDateTime,
    pub is_read: bool,
}

impl NewMessage {
    pub fn create_message(
        chat_id: i32,
        receiver_id: String,
        sender_id: String,
        text: String,
    ) -> Self {
        Self {
            chat_id,
            sender_uid: sender_id,
            receiver_uid: receiver_id,
            text,
            timestamp: Utc::now().naive_utc(),
            is_read: false,
        }
    }
}
