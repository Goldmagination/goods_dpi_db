use actix_web::web;

use super::task_service;

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/task").route("/place", web::post().to(task_service::place_task_handler)),
    );
}
