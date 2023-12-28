use serde::{Serialize, Deserialize};
use diesel::sql_types::*; 
use diesel::QueryableByName; 

#[derive(Debug, QueryableByName, Serialize, Deserialize)]
pub struct ProfessionalProfileDTO {
    #[diesel(sql_type = Integer)]
    pub professional_profiles_id: i32,

    #[diesel(sql_type = Integer)]
    pub category_id: i32,

    #[diesel(sql_type = Text)]
    pub credentials: String,

    #[diesel(sql_type = Bool)]
    pub delivery_enabled: bool,

    #[diesel(sql_type = Nullable<Float8>)]
    pub average_rating: Option<f64>,  // Nullable to handle cases where it might be NaN or NULL

    #[diesel(sql_type = Text)]
    pub street: String,

    #[diesel(sql_type = Text)]
    pub city: String,

    #[diesel(sql_type = Text)]
    pub zip: String,

    #[diesel(sql_type = Float8)]
    pub lng: f64, 

    #[diesel(sql_type = Float8)]
    pub lat: f64,

    #[diesel(sql_type = Text)]
    pub category_name: String,
    
    #[diesel(sql_type = Text)]
    pub professional_name: String,
    
    #[diesel(sql_type = Text)]
    pub service_offering_details: String,  // Holds raw JSON string of service offerings
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceOfferingDTO {
    pub subcategory_id: i32,
    pub subcategory_name: String,
    pub price: f64,  // Ensure this is appropriately represented in your database
}
