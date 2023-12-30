use crate::dal::professional_profile_db;
use crate::db::Pool;
use crate::services::firebase_service::verify_token;
use actix_web::{web, HttpResponse, HttpRequest, Responder, http::header::HeaderValue};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProfessionalProfileQuery {
    subcategory_id: i32,
    lat: f64,
    lng: f64,
}

pub async fn get_professional_profile_handler(req: HttpRequest, query_info: web::Query<ProfessionalProfileQuery>, db_pool: web::Data<Pool>) -> impl Responder {
    let token = _extract_token_from_auth_header(req.headers().get("Authorization"));

    // Verify the token
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
        },
        None => {
            // If no token is present in the request
            return HttpResponse::Unauthorized().body("No token");
        }
    }

    let mut conn = db_pool.get().expect("Failed to get DB connection from pool");
    let subcategory_id = query_info.subcategory_id;
    let lat = query_info.lat;
    let lng = query_info.lng;
    

    match professional_profile_db::search_services(
        subcategory_id, 
        lat, 
        lng, 
        &mut conn,
    ).await { 
        Ok(user_profiles) => {
            if user_profiles.is_empty() {
                // If the list is empty, return 200 OK with an empty list
                HttpResponse::Ok().json(user_profiles)
            } else {
                // If there are user profiles, return them with a 200 OK
                HttpResponse::Ok().json(user_profiles)
            }
        },
        Err(e) => {
            // Log or handle the error more appropriately here
            println!("Error occurred: {:?}", e);
            // Return a 500 Internal Server Error or another appropriate error response
            HttpResponse::InternalServerError().finish()
        },
    }
}

fn _extract_token_from_auth_header(auth_header: Option<&HeaderValue>) -> Option<String> {
    // Extract the Bearer token from the Authorization header
    auth_header?
        .to_str().ok()?
        .split_whitespace().nth(1)
        .map(String::from)
}
