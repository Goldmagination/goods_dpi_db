use chrono::NaiveDateTime;
use diesel::prelude::*;


#[derive(Queryable)]
pub struct Appointment {
    pub id: i32,
    pub customer_id: i32,
    pub professional_profile_id: i32,
    pub date_time: NaiveDateTime,
    pub status: Status,
    pub message: Option<String>,
    pub category_id: i32,
}
