use crate::schema::schema::chat;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
