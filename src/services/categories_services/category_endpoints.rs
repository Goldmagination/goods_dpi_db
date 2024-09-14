use super::category_service;
use actix_web::web;

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/category").route(
        "/subcategories/{category_id}",
        web::get().to(category_service::get_subcategories),
    ));
}
