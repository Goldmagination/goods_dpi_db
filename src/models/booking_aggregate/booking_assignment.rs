use crate::schema::schema::booking_assignments;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct BookingAssignment {
    pub id: i32,
    pub appointment_id: i32,
    pub image_url: String,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = booking_assignments)]
pub struct NewBookingAssignment {
    pub appointment_id: i32,
    pub image_url: String,
}
