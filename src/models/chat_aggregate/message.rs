use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use crate::schema::schema::message;

#[derive(Queryable, Serialize)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub sender_id: i32,
    pub text: String,
    pub timestamp: NaiveDateTime,
    pub is_read: bool,
    pub receiver_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = message)]
pub struct NewMessage {
    pub chat_id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub text: String,
    pub timestamp: NaiveDateTime,
    pub is_read: bool,
}