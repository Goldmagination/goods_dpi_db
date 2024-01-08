use serde::{Serialize, Deserialize};
use diesel::sql_types::*; 
use diesel::QueryableByName; 

#[derive(Debug, QueryableByName, Serialize, Deserialize)]
pub struct ProfessionalProfileDTO {
    #[diesel(sql_type = Integer)]
    pub id: i32,

    #[diesel(sql_type = Nullable<Text>)]
    pub image_url: Option<String>,

    #[diesel(sql_type = Bool)]
    pub delivery_enabled: bool,

    #[diesel(sql_type = Bool)]
    pub remote_available: bool,

    #[diesel(sql_type = Nullable<Float8>)]
    pub average_rating: Option<f64>,  // Nullable to handle cases where it might be NaN or NULL

    #[diesel(sql_type = BigInt)]
    pub review_count: i64,

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
}