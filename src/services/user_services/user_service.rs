use crate::dal::user_db;
use crate::db::Pool;
use crate::models::user_aggregate::new_user::RegistrationData;
use crate::services::firebase_service::create_firebase_user;
use actix_web::web::block;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_user_handler(
    user_email: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let mut conn = match db_pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("Failed to acquire database connection.")
        }
    };

    match user_db::get_user_by_email(&mut conn, &user_email).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn register_user(
    data: web::Json<RegistrationData>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let mut conn = match db_pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("Failed to acquire database connection.")
        }
    };

    // Extract data
    let email = &data.email;
    let password = &data.password;

    // Interact with Firebase to create a new user and get UID
    match create_firebase_user(email, password).await {
        Ok(firebase_response) => {
            match block(move || {
                user_db::save_user_to_database(
                    &mut conn,
                    &data.name,
                    &firebase_response.email,
                    &firebase_response.localId,
                )
            })
            .await
            {
                Ok(Ok(_)) => HttpResponse::Ok().body("User registered successfully"),
                Ok(Err(db_error)) => {
                    HttpResponse::InternalServerError().body(format!("Database error: {}", db_error))
                }
                Err(blocking_error) => HttpResponse::InternalServerError()
                    .body(format!("Task execution error: {}", blocking_error)),
            }
        }
        Err(err) => return HttpResponse::BadRequest().body(format!("Firebase error: {}", err)),
    }
}
