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
    professional_aggregate::business_hour::*,
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
    subcategories,
    business_hours,
};
use chrono::{Utc, Datelike};
use diesel::prelude::*;
use diesel::result::Error;

pub async fn search_services(subcategory_ids_from_user: Vec<i32>, lat_from_user:f64, lng_from_user:f64, conn: &mut PgConnection) -> Result<Vec<ProfessionalProfileDTO>, Error> {
    let radius = 5000.0; // 5 km in meters
    let subcategory_ids_str = subcategory_ids_from_user
    .iter()
    .map(|id| id.to_string())
    .collect::<Vec<_>>()
    .join(", ");

    let raw_sql = format!(r#"
        WITH RelevantProfiles AS (
            SELECT DISTINCT professional_profiles.id
            FROM professional_profiles
            INNER JOIN service_offerings ON professional_profiles.id = service_offerings.professional_profile_id
            WHERE service_offerings.subcategory_id IN ({})
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
            geography(ST_MakePoint({}, {})), 
            {} 
        )
    GROUP BY 
        professional_profiles.id, 
        addresses.street, addresses.city, addresses.zip, addresses.lng, addresses.lat, 
        categories.name, professionals.name;
    
    "#, subcategory_ids_str, lng_from_user, lat_from_user, radius);

    let professional_profiles_from_db: Vec<ProfessionalProfileDTO> = diesel::sql_query(raw_sql)
        .load::<ProfessionalProfileDTO>(conn)?;


    
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
    .load(conn)?;

    let service_offerings = service_offerings_from_db.iter()
    .map(|service_offering| {
        // Query to get category_id from subcategory_id
        let category_id: i32 = subcategories::table
            .filter(subcategories::id.eq(service_offering.subcategory_id))
            .select(subcategories::category_id)
            .first(conn)
            .expect("Failed to retrieve category_id");

        ServiceOfferingDTO::service_offering_to_dto(service_offering, category_id)
    })
    .collect();


    let reviews_from_db = Review::belonging_to(&profile)
    .select(Review::as_select())
    .load::<Review>(conn)
    .optional()?;

    let review_count = reviews_from_db.as_ref().map_or(0, |reviews| reviews.len()) as i64;

    let reviews = reviews_from_db.map(|reviews| {
        reviews.iter().map(|review| { 
            let review_content_assignments = ReviewContentAssignment::belonging_to(review)
                .select(ReviewContentAssignment::as_select())
                .load::<ReviewContentAssignment>(conn)
                .unwrap_or_default();

            ReviewDTO::review_to_dto(review, &review_content_assignments)
        }).collect::<Vec<ReviewDTO>>()
    });
    let today = Utc::now().naive_utc().date();
    let day_of_week = today.weekday().num_days_from_sunday() as i32; // Sunday is 0, Saturday is 6


    let country_code = address.as_ref().map(|addr| &addr.state); // Replace with actual field

    // Determine if today is a holiday
    // let is_today_holiday = match country_code {
    //     Some(code) => is_holiday(code, today),
    //     None => false,
    // };
    let is_today_holiday = false;
    // Fetch business hours
    let business_hour = if is_today_holiday {
        // Fetch business hours for Ferientag (7)
        business_hours::table
            .filter(business_hours::professional_profile_id.eq(profile.id))
            .filter(business_hours::day_of_week.eq(&7))
            .first::<BusinessHours>(conn) // Use BusinessHours here
            .optional()?
    } else {
        // Fetch business hours for the current day of the week
        business_hours::table
            .filter(business_hours::professional_profile_id.eq(profile.id))
            .filter(business_hours::day_of_week.eq(&day_of_week))
            .first::<BusinessHours>(conn) // Use BusinessHours here
            .optional()?
    };
    
    

// Check if the professional is available today
let (opening_time, closing_time) = match business_hour {
    Some(bh) if bh.is_available => (bh.opening_time, bh.closing_time),
    _ => (None, None), // Not available today
};
    let final_profile = ProfessionalProfileDetailDTO {
        id: profile.id,
        professional_name: profile.professional_name,
        opening_time: opening_time,
        closing_time: closing_time,
        image_url: profile.image_url,
        category_name: category.name,
        credentials: profile.credentials,
        delivery_enabled: profile.delivery_enabled,
        remote_available: profile.remote_available,
        average_rating: profile.average_rating,
        address: address,
        service_offerings: service_offerings,
        reviews: reviews,
        review_count: review_count
    };
    Ok(final_profile)
}


