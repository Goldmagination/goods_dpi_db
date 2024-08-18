use super::address_dto::AddressDTO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskDto {
    pub address: Option<AddressDTO>,
    pub title: String,
    pub description: Option<String>,
    pub image_strings: Option<Vec<String>>,
    pub category_id: i32,
    pub is_flexible_timing: bool,
    pub scheduled_date: Option<String>,
    pub scheduled_time: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}
