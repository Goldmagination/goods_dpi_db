use chrono::NaiveDateTime;
use diesel::prelude::*;


#[derive(Queryable)]
pub struct MessageAssignment {
    pub id: i32,
    pub message_id: i32,
    pub image_url: String,
}