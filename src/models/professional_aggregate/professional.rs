use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::schema::professionals;
#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = professionals)]
pub struct Professional {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub user_uid: String,
    pub image_url: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = professionals)]
pub struct NewProfessional {
    pub email: String,
    pub name: String,
    pub user_uid: String
}
