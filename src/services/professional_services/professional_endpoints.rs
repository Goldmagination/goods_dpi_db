use actix_web::{web};

use super::professional_service;

pub fn professional_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/professionals")
            // Define user-related endpoints
            .route("/{professional_email}", web::get().to(
                professional_service::get_professional_handler
            ))
            .route("/register", web::post().to(
                professional_service::register_professional
            ))
    );
}
