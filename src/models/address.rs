use diesel::prelude::*;


#[derive(Queryable)]
pub struct Address {
    pub id: i32,
    pub street: String,
    pub city: String,
    pub state: Country,
    pub zip: String,
    pub lng: f64,
    pub lat: f64
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

