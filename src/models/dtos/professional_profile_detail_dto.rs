use serde::{Serialize, Deserialize};
use super::review_dto::*;
use super::address_dto::*;
use crate::models::professional_aggregate::service_offering::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionalProfileDetailDTO {
    pub id: i32,

    pub image_url: Option<String>, //TODO: Business hours and background image

    pub delivery_enabled: bool,

    pub remote_available: bool,
    
    pub credentials: Option<String>,

    pub average_rating: Option<f64>,  // Nullable to handle cases where it might be NaN or NULL

    pub address: Option<AddressDTO>,

    pub category_name: String,

    pub service_offerings: Vec<ServiceOfferingDTO>,

    pub review_count: i64,

    pub reviews: Option<Vec<ReviewDTO>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceOfferingDTO {
    pub subcategory_name: String,
    pub price: f64,  // Ensure this is appropriately represented in your database
}

impl ServiceOfferingDTO {
pub fn service_offering_to_dto(service_offering: &ServiceOffering) -> ServiceOfferingDTO {
    ServiceOfferingDTO {
        subcategory_name: service_offering.subcategory_name.clone(),
        price: service_offering.price
    }
}}