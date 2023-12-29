use crate::models::
{
    // address::*,
    dtos::professional_profiles_dto::*,
    // professional_aggregate::professional_profile::*,
    // professional_aggregate::service_offering::*
};
// use crate::schema::schema::{
//     professional_profiles,
//     professional_profiles::dsl::*,
//     addresses,
//     addresses::dsl::*,
//     address_assignments,
//     address_assignments::dsl::*,
//     subcategories::dsl::*,
//     subcategories,
//     categories,
//     categories::dsl::*,
//     service_offerings,
//     service_offerings::dsl::*
// };

use diesel::sql_query;
use diesel::prelude::*;
use diesel::result::Error;


pub async fn search_services(subcategory_id_from_user: i32, lat_from_user:f64, lng_from_user:f64, conn: &mut PgConnection) -> Result<Vec<ProfessionalProfileDTO>, Error> {
    let radius = 5000.0; // 5 km in meters
    let raw_sql = r#"
    WITH RelevantProfiles AS (
        SELECT DISTINCT professional_profiles.id
        FROM professional_profiles
        INNER JOIN service_offerings ON professional_profiles.id = service_offerings.professional_profile_id
        WHERE service_offerings.subcategory_id = $1 -- or any other provided subcategory_id
    )
    SELECT
        professional_profiles.id,
        professional_profiles.category_id,
        professional_profiles.credentials,
        professional_profiles.delivery_enabled,
        professional_profiles.average_rating,
        addresses.street, 
        addresses.city, 
        addresses.zip, 
        addresses.lng, 
        addresses.lat, 
        categories.name AS category_name, 
        professionals.name AS professional_name,
        COUNT(review.id) AS review_count
    FROM RelevantProfiles
    INNER JOIN professional_profiles ON RelevantProfiles.id = professional_profiles.id
    INNER JOIN professionals ON professional_profiles.professional_id = professionals.id
    INNER JOIN address_assignments ON professional_profiles.id = address_assignments.professional_profile_id 
    INNER JOIN addresses ON address_assignments.address_id = addresses.id 
    INNER JOIN service_offerings ON professional_profiles.id = service_offerings.professional_profile_id 
    INNER JOIN subcategories ON service_offerings.subcategory_id = subcategories.id
    INNER JOIN categories ON subcategories.category_id = categories.id
    LEFT JOIN review ON professional_profiles.id = review.professional_profile_id 
    WHERE
        ST_DWithin(
            geography(ST_MakePoint(addresses.lng::double precision, addresses.lat::double precision)), 
            geography(ST_MakePoint($2, $3)), 
            $4 
        )
    GROUP BY 
        professional_profiles.id, 
        addresses.street, addresses.city, addresses.zip, addresses.lng, addresses.lat, 
        categories.name, professionals.name;
    
    "#;

    // Query to find professionals based on category and location
    let professional_profiles_from_db: Vec<ProfessionalProfileDTO> = sql_query(raw_sql)
        .bind::<diesel::sql_types::Integer, _>(subcategory_id_from_user)
        .bind::<diesel::sql_types::Double, _>(lng_from_user)
        .bind::<diesel::sql_types::Double, _>(lat_from_user)
        .bind::<diesel::sql_types::Double, _>(radius)
        .load::<ProfessionalProfileDTO>(conn)?; 
    
    // let dto_list: Vec<ProfessionalDTO> = transform_to_dto(professional_profiles_from_db);


    
    Ok(professional_profiles_from_db)
}

// fn transform_to_dto(raw_data: Vec<ProfessionalDTO>) -> Vec<ProfessionalDTO> {
//     raw_data.into_iter().map(|item| {
//         ProfessionalDTO {
//             id: item.id,
//             category_id: item.category_id,
//             credentials: item.credentials,
//             delivery_enabled: item.delivery_enabled,
//             average_rating: item.average_rating,
//             street: item.street,
//             city: item.city,
//             zip: item.zip,
//             lng: item.lng,
//             lat: item.lat,
//             category_name: item.category_name,
//             professional_name: item.professional_name,
//             service_offering_details: item.service_offering_details.into_iter().map(|s| {
//                 ServiceOfferingDTO {
//                     subcategory_id: s.subcategory_id,
//                     subcategory_name: s.subcategory_name,
//                     price: s.price,
//                 }
//             }).collect(),
//         }
//     }).collect()
// }


