use crate::models::
{
    address::*,
    address_assignments::*,
    dtos::professional_profiles_dto::*,
    dtos::professional_profile_detail_dto::*,
    dtos::review_dto::*,
    dtos::address_dto::*,
    professional_aggregate::professional_profile::*,
    professional_aggregate::service_offering::*,
    category_aggregate::category::*,
    review_aggregate::review::*,
    review_aggregate::review_content_assignments::*
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
use crate::schema::schema::{
    addresses,
    professional_profiles, 
    categories,
};
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
        professional_profiles.image_url,
        professional_profiles.delivery_enabled,
        professional_profiles.remote_available,
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

pub async fn get_profile(conn: &mut PgConnection, profile_id: i32)-> Result<ProfessionalProfileDetailDTO, Error> {
    
    let profile = professional_profiles::table.find(profile_id)
    .select(ProfessionalProfile::as_select())
    .first::<ProfessionalProfile>(conn)?;

    let address_from_db = AddressAssignments::belonging_to(&profile)
    .inner_join(addresses::table)
    .select(Address::as_select())
    .first::<Address>(conn).optional();

    let address = match address_from_db {
        Ok(Some(addr)) => Some(AddressDTO::address_to_dto(&addr)),
        Ok(None) => None,
        Err(_) => {
            None
        }
    };


    let category = categories::table.find(profile.category_id)
    .select(Category::as_select())
    .first(conn)?;

    let service_offerings_from_db = ServiceOffering::belonging_to(&profile)
    .select(ServiceOffering::as_select())
    .load(conn)?; //TODO:To DTO

    let service_offerings = service_offerings_from_db.iter()
    .map(ServiceOfferingDTO::service_offering_to_dto)
    .collect();


    let reviews_from_db = Review::belonging_to(&profile)
    .select(Review::as_select())
    .load::<Review>(conn)
    .optional()?;

    let reviews = reviews_from_db.map(|reviews| {
        reviews.iter().map(|review| { 
            let review_content_assignments = ReviewContentAssignment::belonging_to(review)
                .select(ReviewContentAssignment::as_select())
                .load::<ReviewContentAssignment>(conn)
                .unwrap_or_default();

            ReviewDTO::review_to_dto(review, &review_content_assignments)
        }).collect::<Vec<ReviewDTO>>()
    });

    let final_profile = ProfessionalProfileDetailDTO {
        id: profile.id,
        professional_name: profile.professional_name,
        image_url: profile.image_url,
        category_name: category.name,
        credentials: profile.credentials,
        delivery_enabled: profile.delivery_enabled,
        remote_available: profile.remote_available,
        average_rating: profile.average_rating,
        address: address,
        service_offerings: service_offerings,
        reviews: reviews,
        review_count: 0
    };
    Ok(final_profile)
}


