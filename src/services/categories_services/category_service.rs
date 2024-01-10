use crate::dal::category_db;
use crate::db::Pool;
use crate::services::firebase_service::verify_token;
use actix_web::{web, HttpResponse, HttpRequest, Responder, http::header::HeaderValue};


pub async fn get_subcategories(req: HttpRequest, category_id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {

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
        },
        None => {
            // If no token is present in the request
            return HttpResponse::Unauthorized().body("No token");
        }
    }
    let mut conn = db_pool.get().expect("Failed to get DB connection from pool");
    

    match category_db::get_subcategory_by_category_id(
        &mut conn,
        category_id.into_inner(), 
    ).await { 
        Ok(subcategories) => {
            if subcategories.is_empty() {
                HttpResponse::Ok().json(subcategories)
            } else {
                HttpResponse::Ok().json(subcategories)
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