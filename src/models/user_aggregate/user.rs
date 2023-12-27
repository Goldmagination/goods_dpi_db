use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub user_uid: String,
    pub photo_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub user_uid: String
}
