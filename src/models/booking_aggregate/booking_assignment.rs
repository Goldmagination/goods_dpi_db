use diesel::prelude::*;

#[derive(Queryable)]
pub struct BookingAssignment {
    pub id: i32,
    pub appointment_id: i32,
    pub image_url: String,
}
