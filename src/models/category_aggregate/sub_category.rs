use diesel::prelude::*;

#[derive(Queryable)]
pub struct Subcategory {
    pub id: i32,
    pub name: String,
    pub category_id: i32,
}
