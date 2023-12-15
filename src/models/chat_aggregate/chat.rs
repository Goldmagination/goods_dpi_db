use chrono::NaiveDateTime;
use diesel::prelude::*;



#[derive(Queryable)]
pub struct Chat {
    pub id: i32,
    pub user_id: i32,
    pub professional_id: i32,
    pub last_message_time: NaiveDateTime,
}
