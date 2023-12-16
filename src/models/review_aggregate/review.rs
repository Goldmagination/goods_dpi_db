use diesel::prelude::*;
use crate::schema::reviews;

#[derive(Queryable, Insertable)]
pub struct Review {
    pub id: i32,
    pub user_id: i32,
    pub professional_profile_id: i32,
    pub message: String,
    pub rate: f32, // Assuming decimal is represented as a float
}
