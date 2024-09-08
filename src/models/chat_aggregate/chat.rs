use super::message::Message;
use crate::models::booking_aggregate::booking::Booking;
use crate::schema::schema::chat;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ChatItem {
    Message(Message),
    Booking(Booking),
}
impl ChatItem {
    pub fn get_time(&self) -> chrono::DateTime<Utc> {
        match self {
            ChatItem::Message(msg) => DateTime::<Utc>::from_utc(msg.timestamp, Utc),
            ChatItem::Booking(booking) => booking.creation_time,
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = chat)]
pub struct Chat {
    pub id: i32,
    pub last_message_time: NaiveDateTime,
    pub user_uid: String,
    pub professional_profile_uid: String,
}

#[derive(Insertable)]
#[diesel(table_name = chat)]
pub struct NewChat {
    pub user_uid: String,
    pub professional_profile_uid: String,
    pub last_message_time: NaiveDateTime,
}
