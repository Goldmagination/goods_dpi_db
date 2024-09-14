use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingDTO {
    pub id: Option<i32>,
    pub customer_uid: String,
    pub professional_profile_uid: String,
    pub date_time: Option<String>,
    pub end_time: Option<String>,
    pub status: i32,
    pub description: Option<String>,
    pub category_id: i32,
    pub offering_id: i32,
    pub offering_price: f64,
    pub service_offering_name: String,
    pub image_urls: Option<Vec<String>>,
}
