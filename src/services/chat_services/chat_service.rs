use crate::dal::chat_db;
use actix::{fut::ActorFutureExt, Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use chrono::Utc;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{error, info};

const FIREBASE_VALIDATE_TOKEN_URL: &str =
    "https://identitytoolkit.googleapis.com/v1/accounts:lookup";

#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
    message: String,
    receiver_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutgoingMessage {
    sender_uid: String,
    receiver_uid: String,
    text: String,
    timestamp: chrono::DateTime<Utc>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    message: T,
}

#[derive(Serialize)]
struct SuccessMessage {
    message_id: i32,
    content: OutgoingMessage,
}

#[derive(Serialize)]
struct ErrorMessage {
    error: String,
    details: Option<String>,
}

struct ChatWebSocket {
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user_uid: String,
}

impl Actor for ChatWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if !text.is_empty() {
                    self.handle_text_message(text.into_bytes(), ctx);
                }
            }
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Close(reason)) => {
                info!(reason = ?reason, "WebSocket closing");
                ctx.close(reason);
                ctx.stop();
            }
            Ok(_) => (),
            Err(e) => {
                error!("WebSocket error: {:?}", e);
                ctx.stop();
            }
        }
    }
}

impl ChatWebSocket {
    fn handle_text_message(&mut self, text: web::Bytes, ctx: &mut <Self as Actor>::Context) {
        let db_pool = self.db_pool.clone();
        let user_uid = self.user_uid.clone();
        let text_str = String::from_utf8_lossy(&text).to_string();

        let fut = async move {
            match serde_json::from_str::<IncomingMessage>(&text_str) {
                Ok(parsed_message) => {
                    match Self::process_message(db_pool, user_uid, parsed_message).await {
                        Ok(response) => response,
                        Err(e) => {
                            error!("Failed to process message: {:?}", e);
                            // Create a proper error response
                            serde_json::to_string(&ApiResponse {
                                status: "error".to_string(),
                                message: ErrorMessage {
                                    error: "Failed to process message".to_string(),
                                    details: Some(e.to_string()),
                                },
                            })
                            .unwrap()
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to parse message: {:?}", e);
                    serde_json::to_string(&ApiResponse {
                        status: "error".to_string(),
                        message: ErrorMessage {
                            error: "Invalid message format".to_string(),
                            details: Some(e.to_string()),
                        },
                    })
                    .unwrap()
                }
            }
        };

        let fut = actix::fut::wrap_future(fut);
        let fut = fut.map(|result, _actor, ctx: &mut ws::WebsocketContext<Self>| {
            ctx.text(result);
        });

        ctx.spawn(fut);
    }

    async fn process_message(
        db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
        user_uid: String,
        parsed_message: IncomingMessage,
    ) -> Result<String, Error> {
        let message_id = chat_db::send_message(
            db_pool,
            user_uid.clone(),
            parsed_message.receiver_id.clone(),
            parsed_message.message.clone(),
        )
        .await?;

        let outgoing_message = OutgoingMessage {
            sender_uid: user_uid,
            receiver_uid: parsed_message.receiver_id,
            text: parsed_message.message,
            timestamp: Utc::now(),
        };

        let success_message = SuccessMessage {
            message_id,
            content: outgoing_message,
        };

        Ok(serde_json::to_string(&ApiResponse {
            status: "success".to_string(),
            message: success_message,
        })
        .unwrap())
    }
}

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let user_uid = extract_user_uuid(&req)?;

    ws::start(
        ChatWebSocket {
            db_pool: db_pool.clone(),
            user_uid,
        },
        &req,
        stream,
    )
}

pub async fn get_user_chats(
    req: HttpRequest,
    user_uid: web::Path<String>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    if !verify_token_from_request(&req).await {
        return HttpResponse::Unauthorized().body("Invalid or missing token");
    }

    let mut conn = db_pool.get().expect("Failed to get DB connection");
    match chat_db::get_chats_for_user(&mut conn, &user_uid) {
        Ok(chats) => HttpResponse::Ok().json(chats),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse {
            status: "error".to_string(),
            message: ErrorMessage {
                error: "Failed to fetch user chats".to_string(),
                details: None,
            },
        }),
    }
}

#[derive(Deserialize)]
pub struct ChatQuery {
    pub chat_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn read_message(
    req: HttpRequest,
    message_id: web::Path<i32>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    if !verify_token_from_request(&req).await {
        return HttpResponse::Unauthorized().body("Invalid or missing token");
    }
    let mut conn = db_pool.get().expect("Failed to get DB connection");
    match chat_db::read_message(&mut conn, &message_id) {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse {
            status: "error".to_string(),
            message: ErrorMessage {
                error: "Failed to set the message to read".to_string(),
                details: None,
            },
        }),
    }
}

pub async fn get_chat_messages(
    req: HttpRequest,
    query_info: web::Query<ChatQuery>,
    pagination: web::Query<PaginationParams>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    if !verify_token_from_request(&req).await {
        return HttpResponse::Unauthorized().body("Invalid or missing token");
    }

    let chat_id = match query_info.chat_id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Chat ID is required"),
    };

    let limit = pagination.limit.unwrap_or(50);
    let offset = pagination.offset.unwrap_or(0);
    let mut conn = db_pool.get().expect("Failed to get DB connection");

    match chat_db::get_messages_for_chat(&mut conn, chat_id, limit, offset) {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn verify_token_from_request(req: &HttpRequest) -> bool {
    if let Some(token) = extract_token_from_auth_header(req.headers().get("Authorization")) {
        match verify_token(&token).await {
            Ok(is_valid) => is_valid,
            Err(_) => false,
        }
    } else {
        false
    }
}

async fn verify_token(token: &str) -> Result<bool, reqwest::Error> {
    let api_key = env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY must be set");
    let url = format!("{}?key={}", FIREBASE_VALIDATE_TOKEN_URL, api_key);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .json(&serde_json::json!({
            "idToken": token
        }))
        .send()
        .await?;

    Ok(res.status().is_success())
}

fn extract_token_from_auth_header(auth_header: Option<&HeaderValue>) -> Option<String> {
    auth_header?
        .to_str()
        .ok()?
        .split_whitespace()
        .nth(1)
        .map(String::from)
}

fn extract_user_uuid(req: &HttpRequest) -> Result<String, Error> {
    req.match_info()
        .get("user_id")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing user_id parameter"))
        .map(|id| id.to_string())
}
