use super::chat_service;
use actix_web::{middleware, web};

pub fn chat_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/chat")
            .wrap(middleware::Logger::default())
            .route("/ws/{user_id}", web::get().to(chat_service::chat_route))
            .route("/messages", web::get().to(chat_service::get_chat_messages))
            .route(
                "/retrieve/{user_id}",
                web::get().to(chat_service::get_user_chats),
            ),
    );
}
