use diesel::prelude::*;

#[derive(Queryable)]
pub struct OrderSubcategory {
    pub id: i32,
    pub order_id: i32,
    pub subcategory_id: i32,
}
