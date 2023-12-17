use crate::dal::user_db;
use crate::db::Pool;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_user_handler(user_email: web::Path<String>, db_pool: web::Data<Pool>) -> impl Responder {
    let mut conn = db_pool.get().expect("Failed to get DB connection from pool");
    
    match user_db::get_user_by_email(&mut conn, user_email.clone()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

