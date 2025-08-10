use actix::Addr;
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use log::{info, error};
use crate::websocket::{ChatSession, ChatServer};
use crate::middleware::auth::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

/// WebSocket endpoint handler
/// Path: /ws/chat/{user_id}
pub async fn chat_websocket(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<String>,
    chat_server: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    
    // Validate authentication (extract JWT from query params or headers)
    if let Err(e) = validate_websocket_auth(&req, &user_id) {
        error!("WebSocket authentication failed for user {}: {}", user_id, e);
        return Ok(HttpResponse::Unauthorized().body("Authentication required"));
    }
    
    info!("Starting WebSocket for user: {}", user_id);
    
    // Create new session
    let session = ChatSession::new(user_id, chat_server.get_ref().clone());
    
    // Start WebSocket
    ws::start(session, &req, stream)
}

/// Validate WebSocket authentication
fn validate_websocket_auth(req: &HttpRequest, user_id: &str) -> Result<(), String> {
    // Try to get token from Authorization header
    let token = if let Some(auth_header) = req.headers().get("Authorization") {
        auth_header
            .to_str()
            .map_err(|_| "Invalid authorization header")?
            .strip_prefix("Bearer ")
            .ok_or("Invalid authorization format")?
            .to_string()
    } else if let Some(token_param) = req.uri().query()
        .and_then(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .find(|(key, _)| key == "token")
                .map(|(_, value)| value.to_string())
        }) {
        // Fallback to query parameter for WebSocket
        token_param
    } else {
        return Err("No authentication token provided".to_string());
    };
    
    // Validate JWT token
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key".to_string());
    
    let validation = Validation::new(Algorithm::HS256);
    
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ) {
        Ok(token_data) => {
            // Verify user_id matches token
            if token_data.claims.sub != user_id {
                Err("User ID mismatch".to_string())
            } else {
                Ok(())
            }
        }
        Err(e) => Err(format!("Invalid token: {}", e))
    }
}

/// Health check endpoint for WebSocket service
pub async fn websocket_health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "websocket"
    }))
}