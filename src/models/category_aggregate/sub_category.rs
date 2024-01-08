use diesel::prelude::*;
use crate::schema::schema::subcategories;
use super::category::Category;


#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Category, foreign_key=category_id))]
#[diesel(table_name = subcategories)]
pub struct Subcategory {
    pub id: i32,
    pub name: String,
    pub category_id: i32,
}
