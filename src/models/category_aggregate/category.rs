use diesel::prelude::*;
use crate::schema::schema::categories;
#[derive(Queryable,Identifiable, Selectable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
