use crate::schema::schema::message;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
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
