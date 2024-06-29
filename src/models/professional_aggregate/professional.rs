use crate::schema::schema::professionals;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = professionals)]
pub struct Professional {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub image_url: Option<String>,
    pub user_uid: String,
}

#[derive(Insertable)]
#[diesel(table_name = professionals)]
pub struct NewProfessional {
    pub email: String,
    pub name: String,
    pub user_uid: String,
}
