use actix_web::web;

pub fn professional_profile_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profiles")
            // Define user-related endpoints
            .route(
                "/search",
                web::get()
                    .to(super::professional_profile_service::get_professional_profile_handler),
            )
            .route(
                "/{profile_email}",
                web::get().to(super::professional_profile_service::get_profile_by_id),
            ),
    );
}
