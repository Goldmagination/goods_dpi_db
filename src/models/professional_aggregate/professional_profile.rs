use diesel::prelude::*;


#[derive(Queryable)]
pub struct ProfessionalProfile {
    pub id: i32,
    pub professional_id: i32,
    pub category_id: i32,
    pub credentials: String,
    pub delivery_enabled: bool,
}