use serde::{Deserialize, Serialize};

use crate::models::address::Address;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressDTO {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

pub enum Country {
    Germany,
    USA,
    // ...
}

impl Country {
    pub fn as_str(&self) -> &'static str {
        match self {
            Country::Germany => "DE",
            Country::USA => "US",
            // ...
        }
    }
}
impl AddressDTO {
    pub fn address_to_dto(address: &Address) -> AddressDTO {
        AddressDTO {
            street: address.street.clone(),
            city: address.city.clone(),
            state: address.state.clone(),
            zip: address.zip.clone(),
            lat: Some(address.lat.clone()),
            lng: Some(address.lng.clone()),
        }
    }
}
