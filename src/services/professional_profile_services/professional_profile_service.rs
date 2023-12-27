use crate::dal::professional_profile_db;
use crate::db::Pool;
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::block;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProfessionalProfileQuery {
    subcategory_id: i32,
    lat: f64,
    lng: f64,
}

pub async fn get_professional_profile_handler(query_info: web::Json<ProfessionalProfileQuery>, db_pool: web::Data<Pool>) -> impl Responder {
    print!("i am here");
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
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}