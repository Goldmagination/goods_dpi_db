use diesel::prelude::*;

#[derive(Queryable)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
}
