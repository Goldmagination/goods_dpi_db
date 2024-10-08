use crate::dal::booking_db;
use crate::dal::professional_profile_db;
use crate::models::dtos::booking_dto::BookingDTO;
use crate::services::firebase_service::{extract_uid_from_firebase_token, verify_token};
use actix_web::{http::header::HeaderValue, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProfessionalProfileQuery {
    subcategory_ids: String,
    lat: f64,
    lng: f64,
}

pub async fn get_professional_profile_handler(
    req: HttpRequest,
    query_info: web::Query<ProfessionalProfileQuery>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    // Verify the token
    let token = _extract_token_from_auth_header(req.headers().get("Authorization"));
    match token {
        Some(t) => {
            if let Ok(is_valid) = verify_token(&t).await {
                if !is_valid {
                    // If the token is not valid, return an unauthorized response
                    return HttpResponse::Unauthorized().body("Invalid token");
                }
            } else {
                // If token verification failed due to some error
                return HttpResponse::InternalServerError().finish();
            }
        }
        None => {
            // If no token is present in the request
            return HttpResponse::Unauthorized().body("No token");
        }
    }
    let mut conn = db_pool
        .get()
        .expect("Failed to get DB connection from pool");
    let lat = query_info.lat;
    let lng = query_info.lng;
    let subcategory_ids: Result<Vec<i32>, _> = query_info
        .subcategory_ids
        .split(',')
        .map(|s| s.parse::<i32>())
        .collect();

    let subcategory_ids = match subcategory_ids {
        Ok(ids) => ids,
        Err(_) => return HttpResponse::BadRequest().body("Invalid subcategory IDs"),
    };

    match professional_profile_db::search_services(subcategory_ids, lat, lng, &mut conn).await {
        Ok(user_profiles) => {
            if user_profiles.is_empty() {
                // If the list is empty, return 200 OK with an empty list
                HttpResponse::Ok().json(user_profiles)
            } else {
                // If there are user profiles, return them with a 200 OK
                HttpResponse::Ok().json(user_profiles)
            }
        }
        Err(e) => {
            // Log or handle the error more appropriately here
            println!("Error occurred: {:?}", e);
            // Return a 500 Internal Server Error or another appropriate error response
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_profile_by_id(
    req: HttpRequest,
    profile_id: web::Path<i32>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    // Verify the token
    let token = _extract_token_from_auth_header(req.headers().get("Authorization"));

    match token {
        Some(t) => {
            if let Ok(is_valid) = verify_token(&t).await {
                if !is_valid {
                    // If the token is not valid, return an unauthorized response
                    return HttpResponse::Unauthorized().body("Invalid token");
                }
            } else {
                // If token verification failed due to some error
                return HttpResponse::InternalServerError().finish();
            }
        }
        None => {
            // If no token is present in the request
            return HttpResponse::Unauthorized().body("No token");
        }
    }
    let mut conn = db_pool
        .get()
        .expect("Failed to get DB connection from pool");

    match professional_profile_db::get_profile(&mut conn, profile_id.into_inner()).await {
        Ok(professional_profile_dto) => HttpResponse::Ok().json(professional_profile_dto),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
fn _extract_token_from_auth_header(auth_header: Option<&HeaderValue>) -> Option<String> {
    // Extract the Bearer token from the Authorization header
    auth_header?
        .to_str()
        .ok()?
        .split_whitespace()
        .nth(1)
        .map(String::from)
}
pub async fn book_service_handler(
    req: HttpRequest,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    booking_dto: web::Json<BookingDTO>,
    profile_id: web::Path<i32>,
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
                        profile_id.into_inner(),
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
