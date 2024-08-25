use chrono::NaiveDateTime;
use diesel::prelude::*; // Assuming Diesel ORM is used
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Booking {
    pub id: i32,
    pub customer_id: i32,
    pub professional_profile_id: i32,
    pub date_time: Option<NaiveDateTime>,
    pub status: i32,
    pub description: Option<String>,
    pub category_id: i32,
    pub offering_id: i32,
}
