use crate::dal::chat_db;
use actix::{fut::ActorFutureExt, Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use log::{error, info};
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use uuid::Uuid;

const FIREBASE_VALIDATE_TOKEN_URL: &str =
    "https://identitytoolkit.googleapis.com/v1/accounts:lookup";

#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
    message: String,
    receiver_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutgoingMessage {
    sender_id: Uuid,
    receiver_id: Uuid,
    text: String,
}

struct ChatWebSocket {
    db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    user_uuid: Uuid,
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
                info!("WebSocket closing: {:?}", reason);
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
        let db_pool = Arc::clone(&self.db_pool);
        let user_uuid = self.user_uuid;
        let text_str = String::from_utf8_lossy(&text).to_string();

        let fut = async move {
            match serde_json::from_str::<IncomingMessage>(&text_str) {
                Ok(parsed_message) => {
                    match Self::process_message(db_pool.into(), user_uuid, parsed_message).await {
                        Ok(response) => response,
                        Err(e) => {
                            error!("Failed to process message: {:?}", e);
                            "Error: Failed to process message".to_string()
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to parse message: {:?}", e);
                    "Error: Invalid message format".to_string()
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
        user_uuid: Uuid,
        parsed_message: IncomingMessage,
    ) -> Result<String, Error> {
        chat_db::send_message(
            db_pool.clone(),
            user_uuid,
            parsed_message.receiver_id,
            parsed_message.message,
        )
        .await?; // Ensure you await the async function call

        Ok("Message processed successfully".to_string())
    }
}

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let user_uuid = extract_user_uuid(&req)?;

    ws::start(
        ChatWebSocket {
            db_pool: Arc::new(db_pool.get_ref().clone()),
            user_uuid,
        },
        &req,
        stream,
    )
}

pub async fn get_user_chats(
    req: HttpRequest,
    user_uuid: web::Path<Uuid>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    if !verify_token_from_request(&req).await {
        return HttpResponse::Unauthorized().body("Invalid or missing token");
    }

    let mut conn = db_pool.get().expect("Failed to get DB connection");
    match chat_db::get_chats_for_user(&mut conn, &user_uuid) {
        Ok(chats) => HttpResponse::Ok().json(chats),
        Err(_) => HttpResponse::InternalServerError().finish(),
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

// You need to implement this function to extract the user UUID from the request
fn extract_user_uuid(req: &HttpRequest) -> Result<Uuid, Error> {
    // Implementation depends on how you're storing the user UUID in the request
    // This is just a placeholder
    unimplemented!()
}
