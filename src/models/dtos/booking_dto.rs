use serde::{Deserialize, Serialize};

use crate::errors::booking_errors::BookingError;

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
    pub image_urls: Option<Vec<String>>,
}