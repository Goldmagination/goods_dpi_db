use diesel::prelude::*;

#[derive(Queryable)]
pub struct AddressAssignment {
    pub id: i32,
    pub professional_profile_id: i32,
    pub address_id: i32,
}
