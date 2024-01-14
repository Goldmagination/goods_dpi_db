use std::env;

use actix::{Actor, StreamHandler};
use actix_web::{error, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use reqwest::header::HeaderValue;
use crate::dal::chat_db;
use serde::Deserialize;
use serde_json::Value;

const FIREBASE_VALIDATE_TOKEN_URL: &str = "https://identitytoolkit.googleapis.com/v1/accounts:lookup";
// Assuming you have a way to identify a user, e.g., a token or user ID passed in the message
struct ChatWebSocket {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    user_id: i32,
}

impl Actor for ChatWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

use serde_json::json;

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if !text.is_empty() {
                    let db_pool = self.db_pool.clone();
                    let user_id = self.user_id;
                    let parsed_message: Value = serde_json::from_str(&text).expect("Invalid JSON format");
                    println!("{:#?}", parsed_message);

                    let message_text = parsed_message["message"].as_str().expect("Message text missing").to_string();
                    let message_text_clone = message_text.clone();
                    let receiver_id = parsed_message["receiver_id"].as_i64().expect("Receiver ID missing") as i32;

                    
                    actix::spawn(async move {
                        let mut conn = db_pool.get().expect("Failed to get DB connection");
                        let _ = chat_db::send_message(&mut conn, user_id, receiver_id, &message_text_clone).await;
                    });

                    // Create a structured message
                    let message = json!({
                        "sender_id": user_id,
                        "receiver_id": receiver_id,
                        "text": message_text,
                    }).to_string();

                    // Send the structured message
                    ctx.text(message);
                }
            }
            _ => (),
        }
    }
}







pub async fn chat_route(
    req: HttpRequest,
    user_id: web::Path<i32>, 
    stream: web::Payload, 
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>
) -> Result<HttpResponse, Error> {
    match (user_id) {
       user_id => {
            ws::start(
                ChatWebSocket { 
                    db_pool: db_pool.get_ref().clone(), 
                    user_id: user_id.into_inner(),
                }, 
                &req, 
                stream
            )
        },
        _ => Err(error::ErrorBadRequest("Missing required parameters")),
    }
}



pub async fn get_user_chats(
    req: HttpRequest,
    user_id: web::Path<i32>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let token = _extract_token_from_auth_header(req.headers().get("Authorization"));
    match token {
        Some(t) => {
            if let Ok(is_valid) = verify_token(&t).await {
                if !is_valid {
                    // If the token is not valid, return an unauthorized response
                    return HttpResponse::Unauthorized().body("Invalid token");
                }
            } else {
                // If token verification failed due to some error
                return HttpResponse::InternalServerError().finish();
            }
        },
        None => {
            // If no token is present in the request
            return HttpResponse::Unauthorized().body("No token");
        }
    }

    let mut conn = db_pool.get().expect("Failed to get DB connection");
    match chat_db::get_chats_for_user(&mut conn, *user_id) {
        Ok(chats) => HttpResponse::Ok().json(chats),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
pub struct ChatQuery {
    sender_id: i32,
    chat_id: Option<i32>,
    professional_profile_id: i32,
}

pub async fn get_chat_messages(
    req: HttpRequest,
    query_info: web::Query<ChatQuery>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let token = _extract_token_from_auth_header(req.headers().get("Authorization"));
    match token {
        Some(t) => {
            if let Ok(is_valid) = verify_token(&t).await {
                if !is_valid {
                    // If the token is not valid, return an unauthorized response
                    return HttpResponse::Unauthorized().body("Invalid token");
                }
            } else {
                // If token verification failed due to some error
                return HttpResponse::InternalServerError().finish();
            }
        },
        None => {
            // If no token is present in the request
            return HttpResponse::Unauthorized().body("No token");
        }
    }
    let sender_id = query_info.sender_id;
    let professional_profile_id = query_info.professional_profile_id;
    let chat_id = query_info.chat_id;
    let mut conn = db_pool.get().expect("Failed to get DB connection");

    match chat_id {
        Some(chat_id) => {
            match chat_db::get_messages_for_chat(&mut conn, chat_id) {
                Ok(messages) => HttpResponse::Ok().json(messages),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        },
        None => {
            match chat_db::get_messages_for_sender_and_receiver(&mut conn, sender_id, professional_profile_id) {
                Ok(messages) => HttpResponse::Ok().json(messages),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        },
    }
}





pub async fn verify_token(token: &str) -> Result<bool, reqwest::Error> {
    let api_key = env::var("FIREBASE_API_KEY")
        .expect("FIREBASE_API_KEY must be set");
    let url = format!("{}?key={}", FIREBASE_VALIDATE_TOKEN_URL, api_key);
    let client = reqwest::Client::new();
    let res = client.post(&url)
        .json(&serde_json::json!({
            "idToken": token
        }))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(true) // Token is valid
    } else {
        Ok(false) // Token is invalid
    }
}
fn _extract_token_from_auth_header(auth_header: Option<&HeaderValue>) -> Option<String> {
    // Extract the Bearer token from the Authorization header
    auth_header?
        .to_str().ok()?
        .split_whitespace().nth(1)
        .map(String::from)
}