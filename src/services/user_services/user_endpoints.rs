use actix_web::web;

use super::user_service;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // Define user-related endpoints
            .route(
                "/{user_email}",
                web::get().to(user_service::get_user_handler),
            )
            .route("/register", web::post().to(user_service::register_user)),
    );
}
