use diesel::prelude::*;

use crate::schema::schema::message_assignments;

#[derive(Queryable)]
pub struct MessageAssignment {
    pub id: i32,
    pub message_id: i32,
    pub image_url: String,
}

#[derive(Insertable)]
#[diesel(table_name = message_assignments)]
pub struct NewMessageAssignment {
    pub message_id: i32,
    pub image_url: String,
}
