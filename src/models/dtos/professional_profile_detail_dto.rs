use super::address_dto::*;
use super::review_dto::*;
use crate::models::professional_aggregate::service_offering::*;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionalProfileDetailDTO {
    pub id: i32,

    pub uid: String,

    pub professional_name: String,

    pub category_id: i32,

    pub image_url: Option<String>, //TODO: Business hours and background image

    pub opening_time: Option<NaiveTime>,

    pub closing_time: Option<NaiveTime>,

    pub delivery_enabled: bool,

    pub remote_available: bool,

    pub credentials: Option<String>,

    pub average_rating: Option<f64>, // Nullable to handle cases where it might be NaN or NULL

    pub address: Option<AddressDTO>,

    pub category_name: String,

    pub service_offerings: Vec<ServiceOfferingDTO>,

    pub review_count: i64,

    pub reviews: Option<Vec<ReviewDTO>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceOfferingDTO {
    pub id: i32,
    pub subcategory_id: i32,
    pub subcategory_name: String,
    pub category_id: i32,
    pub price: f64,
}

impl ServiceOfferingDTO {
    pub fn service_offering_to_dto(
        service_offering: &ServiceOffering,
        category_id: i32,
    ) -> ServiceOfferingDTO {
        ServiceOfferingDTO {
            id: service_offering.id,
            subcategory_id: service_offering.subcategory_id,
            subcategory_name: service_offering.subcategory_name.clone(),
            category_id: category_id,
            price: service_offering.price,
        }
    }
}
