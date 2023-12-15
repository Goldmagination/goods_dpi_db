use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub creation_time: NaiveDateTime,
    pub description: String,
    pub address_id: i32,
}