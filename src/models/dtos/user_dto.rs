use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::user_aggregate::user::User;
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub image_url: Option<String>,
    pub user_uid: String,
    pub unread_messages: i64,
    pub active_bookings: i64,
}
impl UserDTO {
    pub fn new(user: User, unread_messages: i64, active_bookings: i64) -> Self {
        UserDTO {
            name: user.name,
            email: user.email,
            image_url: user.image_url,
            user_uid: user.user_uid,
            unread_messages,
            active_bookings,
        }
    }
}
