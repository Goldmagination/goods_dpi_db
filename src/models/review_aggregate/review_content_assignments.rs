use diesel::prelude::*;
use crate::schema::reviews;

#[derive(Queryable, Insertable)]
pub struct ReviewContentAssignments {
    pub id: i32,
    pub review_id: i32,
    pub image_url: String,
}