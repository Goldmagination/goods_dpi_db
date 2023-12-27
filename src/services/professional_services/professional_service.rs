use crate::dal::professional_db;
use crate::db::Pool;
use crate::models::professional_aggregate::new_professional::RegistrationData;
use crate::services::firebase_service::create_firebase_user;
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::block;

pub async fn get_professional_handler(professional_email: web::Path<String>, db_pool: web::Data<Pool>) -> impl Responder {
    let mut conn = db_pool.get().expect("Failed to get DB connection from pool");
    
    match professional_db::get_professional_by_email(&mut conn, professional_email.into_inner()) {
        Ok(professional) => HttpResponse::Ok().json(professional),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn register_professional(data: web::Json<RegistrationData>, db_pool: web::Data<Pool>) -> impl Responder {
    let mut conn = db_pool.get().expect("Failed to get DB connection from pool");

    // Extract data
    let email = &data.email;
    let password = &data.password;


    // Interact with Firebase to create a new user and get UID
    match create_firebase_user(email, password).await {
        Ok(firebase_response) => {
            match block(move || {
                professional_db::save_professional_to_database(&mut conn, &data.name, &firebase_response.email, &firebase_response.localId)
            }).await {
                Ok(_) => HttpResponse::Ok().body("Professional registered successfully"),
                Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
            }
        },
        Err(err) => return HttpResponse::BadRequest().body(format!("Firebase error: {}", err)),
    }
}
