use crate::schema::schema::bookings;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Booking {
    pub id: i32,
    pub customer_uid: String,
    pub professional_profile_uid: String,
    pub date_time: Option<DateTime<Utc>>,
    pub status: i32,
    pub description: Option<String>,
    pub category_id: i32,
    pub end_time: Option<DateTime<Utc>>,
    pub service_offering_id: Option<i32>,
    pub offering_price: f64,
    pub chat_id: i32,
    pub creation_time: DateTime<Utc>,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = bookings)]
pub struct NewBooking {
    pub customer_uid: String,
    pub professional_profile_uid: String,
    pub date_time: Option<DateTime<Utc>>,
    pub status: i32,
    pub description: Option<String>,
    pub category_id: i32,
    pub service_offering_id: Option<i32>,
    pub offering_price: f64,
    pub chat_id: i32,
    pub creation_time: DateTime<Utc>,
}
