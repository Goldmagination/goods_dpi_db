use actix_web::{web};
use super::professional_profile_service;

pub fn professional_profile_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/services")
            // Define user-related endpoints
            .route("/search", web::get().to(
                professional_profile_service::get_professional_profile_handler
            ))
    );
}

