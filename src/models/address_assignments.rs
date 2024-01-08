use diesel::prelude::*;
use crate::schema::schema::address_assignments;
use crate::models::
{
    address::*,
    
    professional_aggregate::professional_profile::*,
    
};

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Address))]
#[diesel(belongs_to(ProfessionalProfile))]
#[diesel(primary_key(address_id, professional_profile_id))]
#[diesel(table_name = address_assignments)]
pub struct AddressAssignments {
    pub id: i32,
    pub address_id: i32,
    pub professional_profile_id: i32,
}