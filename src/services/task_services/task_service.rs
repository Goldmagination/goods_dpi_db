use crate::dal::task_db;
use crate::models::dtos::task_dto::TaskDto;
use crate::services::firebase_service::{extract_uid_from_firebase_token, verify_token};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub async fn place_task_handler(
    req: HttpRequest,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    task_dto: web::Json<TaskDto>,
) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth_header| auth_header.split_whitespace().nth(1)); // Assuming "Bearer <token>"

    match token {
        Some(t) => match verify_token(&t).await {
            Ok(is_valid) if is_valid => match extract_uid_from_firebase_token(&t).await {
                Ok(user_uid) => {
                    match task_db::place_task(db_pool.clone(), user_uid, task_dto.into_inner())
                        .await
                    {
                        Ok(task) => HttpResponse::Ok().json(task),
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    }
                }
                Err(_) => HttpResponse::Unauthorized().body("Invalid user"),
            },
            Ok(_) => HttpResponse::Unauthorized().body("Invalid token"),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        None => HttpResponse::Unauthorized().body("No token"),
    }
}
