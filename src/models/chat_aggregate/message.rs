use crate::schema::schema::message;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub text: String,
    pub timestamp: NaiveDateTime,
    pub is_read: bool,
    pub receiver_uid: Uuid,
    pub sender_uid: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = message)]
pub struct NewMessage {
    pub chat_id: i32,
    pub sender_uid: Uuid,
    pub receiver_uid: Uuid,
    pub text: String,
    pub timestamp: NaiveDateTime,
    pub is_read: bool,
}
