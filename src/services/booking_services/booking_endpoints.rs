use super::booking_service;
use actix_web::web;

pub fn booking_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/booking").route(
        "/{professional_id}/book-service",
        web::get().to(booking_service::get_subcategories),
    ));
}
