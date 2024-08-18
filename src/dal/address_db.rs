use crate::models::address::{Address, NewAddress};
use crate::models::dtos::address_dto::AddressDTO;
use crate::schema::schema::addresses::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub fn find_address(
    conn: &mut PgConnection,
    address_dto: &AddressDTO,
) -> Result<Option<i32>, Error> {
    let existing_address = addresses
        .filter(street.eq(&address_dto.street))
        .filter(city.eq(&address_dto.city))
        .filter(state.eq(&address_dto.state))
        .filter(zip.eq(&address_dto.zip))
        .first::<Address>(conn)
        .optional()?;

    Ok(existing_address.map(|addr| addr.id))
}
pub fn insert_address(conn: &mut PgConnection, address_dto: &AddressDTO) -> Result<i32, Error> {
    let new_address = NewAddress {
        street: address_dto.street.clone(),
        city: address_dto.city.clone(),
        state: address_dto.state.clone(),
        zip: address_dto.zip.clone(),
        lng: address_dto.lng.unwrap_or(0.0),
        lat: address_dto.lat.unwrap_or(0.0),
    };

    diesel::insert_into(addresses)
        .values(&new_address)
        .returning(id)
        .get_result::<i32>(conn)
}
