use actix_web::{web};
use super::category_service;

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/category")
            .route("/subcategories/{category_id}", web::get().to(
                category_service::get_subcategories
            ))
    );
}
