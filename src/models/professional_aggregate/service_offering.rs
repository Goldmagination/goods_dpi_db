use diesel::prelude::*;
use crate::schema::schema::service_offerings;
use crate::models::professional_aggregate::professional_profile::ProfessionalProfile;

#[derive(Queryable, Identifiable, Selectable, Associations)]
#[diesel(belongs_to(ProfessionalProfile))]
#[diesel(primary_key(professional_profile_id))]
#[diesel(table_name = service_offerings)]
pub struct ServiceOffering {
    pub id: i32,
    pub professional_profile_id: i32,
    pub subcategory_id: i32,
    pub price: f64, // Assuming `decimal` maps to `f64`
    pub subcategory_name: String,
}
