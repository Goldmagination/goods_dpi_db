use crate::errors::booking_errors::BookingError;
use crate::models::booking_aggregate::booking::{Booking, NewBooking};
use crate::models::booking_aggregate::booking_assignment::{
    BookingAssignment, NewBookingAssignment,
};
use crate::models::booking_aggregate::booking_status::BookingStatus;
use crate::models::dtos::booking_dto::BookingDTO;
use crate::schema::schema::{booking_assignments, bookings};
use actix_web::web;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;

pub async fn place_booking(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user_uid: String,
    booking_dto: BookingDTO, // Assuming you have the BookingDTO structure defined
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
                .and_then(|dt_str| DateTime::parse_from_rfc3339(dt_str).ok()) // Parse if not null
                .map(|dt| dt.with_timezone(&Utc));

            conn.transaction::<_, BookingError, _>(|conn| {
                let new_booking = NewBooking {
                    customer_uid: booking_dto.customer_uid,
                    professional_profile_uid: booking_dto.professional_profile_uid,
                    date_time,
                    status: booking_dto.status,
                    description: booking_dto.description,
                    category_id: booking_dto.category_id,
                    service_offering_id: Some(booking_dto.offering_id),
                    offering_price: booking_dto.offering_price,
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
        Err(e) => return Err(e),
    };

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

    let new_booking_assignment = NewBookingAssignments {
        booking_id,
        image_url,
    };

    diesel::insert_into(booking_assignment::table)
        .values(&new_booking_assignment)
        .execute(&mut conn)
        .map_err(BookingError::DieselError)?;

    Ok(())
}
