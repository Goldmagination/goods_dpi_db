use diesel::prelude::*;

#[derive(Queryable)]
pub struct AppointmentAssignment {
    pub id: i32,
    pub appointment_id: i32,
    pub image_url: String,
}
