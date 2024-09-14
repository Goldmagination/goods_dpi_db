use crate::dal::booking_db;
use crate::models::dtos::booking_dto::BookingDTO;
use crate::services::firebase_service::{extract_uid_from_firebase_token, verify_token};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub async fn book_service_handler(
    req: HttpRequest,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    booking_dto: web::Json<BookingDTO>,
) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth_header| auth_header.split_whitespace().nth(1)); // Assuming "Bearer <token>"

    match token {
        Some(t) => match verify_token(&t).await {
            Ok(is_valid) if is_valid => match extract_uid_from_firebase_token(&t).await {
                Ok(user_uid) => {
                    match booking_db::place_booking(
                        db_pool.clone(),
                        user_uid,
                        booking_dto.into_inner(),
                    )
                    .await
                    {
                        Ok(booking) => HttpResponse::Ok().json(booking),
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    }
                }
                Err(_) => HttpResponse::Unauthorized().body("Invalid user"),
            },
            Ok(_) => HttpResponse::Unauthorized().body("Invalid token"),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        None => HttpResponse::Unauthorized().body("No token"),
    }
}
