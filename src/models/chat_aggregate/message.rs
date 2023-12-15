use chrono::NaiveDateTime;
use diesel::prelude::*;


#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub sender_id: i32,
    pub text: String,
    pub timestamp: NaiveDateTime,
    pub is_read: bool,
}