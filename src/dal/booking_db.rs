use super::chat_db::get_or_create_chat;
use crate::errors::booking_errors::BookingError;
use crate::models::booking_aggregate::booking::{Booking, NewBooking};
use crate::models::booking_aggregate::booking_assignment::NewBookingAssignment;
use crate::models::dtos::booking_dto::BookingDTO;
use crate::schema::schema::{booking_assignments, bookings};
use actix_web::web;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub async fn place_booking(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user_uid: String,
    booking_dto: BookingDTO,
    profile_id: i32,
) -> Result<Booking, BookingError> {
    let booking_creation_result = web::block({
        let db_pool = db_pool.clone();
        move || -> Result<Booking, BookingError> {
            let mut conn = db_pool
                .get()
                .map_err(|e| BookingError::DatabasePoolError(e.to_string()))?;

            let date_time: Option<DateTime<Utc>> = booking_dto
                .date_time
                .as_ref()
                .and_then(|dt_str| DateTime::parse_from_rfc3339(dt_str).ok())
                .map(|dt| dt.with_timezone(&Utc));

            let chat_id =
                get_or_create_chat(&mut conn, &user_uid, &booking_dto.professional_profile_uid)?;

            conn.transaction::<_, BookingError, _>(|conn| {
                let new_booking = NewBooking {
                    customer_uid: user_uid.clone(),
                    professional_profile_uid: booking_dto.professional_profile_uid.clone(),
                    date_time,
                    status: booking_dto.status,
                    description: booking_dto.description,
                    category_id: booking_dto.category_id,
                    service_offering_id: Some(booking_dto.offering_id),
                    service_offering_name: Some(booking_dto.service_offering_name),
                    offering_price: booking_dto.offering_price,
                    chat_id,
                    creation_time: Utc::now(),
                };

                diesel::insert_into(bookings::table)
                    .values(&new_booking)
                    .get_result(conn)
                    .map_err(BookingError::DieselError)
            })
        }
    })
    .await
    .map_err(|e| BookingError::BlockingError(format!("Blocking error: {}", e)))?;

    let booking = match booking_creation_result {
        Ok(booking) => booking,
        Err(e) => {
            println!("Error creating booking: {:?}", e);
            return Err(e);
        }
    };

    if let Some(image_urls) = booking_dto.image_urls {
        for image_url in image_urls {
            let db_pool = db_pool.clone();
            let booking_id = booking.id;
            let image_url_clone = image_url.clone();

            web::block(move || insert_booking_assignment(&db_pool, booking_id, image_url_clone))
                .await
                .map_err(|e| BookingError::BlockingError(format!("Blocking error: {}", e)))?
                .map_err(|e| {
                    BookingError::BlockingError(format!("Task image save error: {}", e))
                })?;
        }
    }

    Ok(booking)
}

fn insert_booking_assignment(
    db_pool: &Pool<ConnectionManager<PgConnection>>,
    booking_id: i32,
    image_url: String,
) -> Result<(), BookingError> {
    let mut conn = db_pool
        .get()
        .map_err(|e| BookingError::DatabasePoolError(e.to_string()))?;

    let new_booking_assignment = NewBookingAssignment {
        appointment_id: booking_id,
        image_url,
    };

    diesel::insert_into(booking_assignments::table)
        .values(&new_booking_assignment)
        .execute(&mut conn)
        .map_err(BookingError::DieselError)?;

    Ok(())
}
// pub fn get_booking_by_user(
//     conn: &mut PgConnection,
//     user_uid: &str,
// ) -> Result<Vec<BookingDto>, BookingError> {
//     let bookings: Vec<(Booking, Option<BookingAssignments>)> = bookings::table
//         .filter(bookings::customer_uid.eq(user_uid))
//         .left_join(
//             booking_assignments::table.on(bookings::id.eq(booking_assignments::appointment_id)),
//         )
//         .load(conn)?;

//     let booking_dtos: Vec<BookingDto> = bookings
//         .into_iter()
//         .map(|(booking, booking_assignment)| {
//             let image_strings = booking_assignment.map(|assignment| vec![assignment.image_url]);

//             BookingDto {
//                 title: booking.title.clone(),
//                 description: booking.description.clone(),
//                 image_strings,
//                 category_id: booking.category_id,
//                 is_flexible_timing: booking.is_flexible_timing,
//                 scheduled_date: booking.scheduled_date.map(|d| d.to_string()),
//                 scheduled_time: booking.scheduled_time.map(|t| t.to_string()),
//                 min_price: booking.min_price,
//                 max_price: booking.max_price,
//             }
//         })
//         .collect();

//     Ok(booking_dtos)
// }
