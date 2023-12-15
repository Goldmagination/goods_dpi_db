use diesel::prelude::*;

#[derive(Queryable)]
pub struct Professional {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub user_uid: String,
}
