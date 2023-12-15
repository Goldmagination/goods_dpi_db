use diesel::prelude::*;

#[derive(Queryable)]
pub struct ServiceOffering {
    pub id: i32,
    pub professional_profile_id: i32,
    pub subcategory_id: i32,
    pub price: f64, // Assuming `decimal` maps to `f64`
}
