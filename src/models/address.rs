use diesel::prelude::*;


#[derive(Queryable)]
pub struct Address {
    pub id: i32,
    pub street: String,
    pub city: String,
    pub state: Country,
    pub zip: String,
}
