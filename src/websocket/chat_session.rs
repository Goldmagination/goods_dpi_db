use actix::prelude::*;
use actix_web_actors::ws;
use log::{debug, error, info, warn};
use std::time::{Duration, Instant};
use super::messages::*;
use super::chat_server::{ChatServer, SessionMessage};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Individual WebSocket session for a user
pub struct ChatSession {
    /// User ID
    pub user_id: String,
    
    /// Chat server address
    pub server: Addr<ChatServer>,
    
    /// Client must send ping at least once per CLIENT_TIMEOUT
    pub heartbeat: Instant,
}

impl ChatSession {
    pub fn new(user_id: String, server: Addr<ChatServer>) -> Self {
        ChatSession {
            user_id,
            server,
            heartbeat: Instant::now(),
        }
    }
    
    /// Send ping to client every HEARTBEAT_INTERVAL
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // Check if client sent us something recently
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                warn!("WebSocket client heartbeat failed for user {}, disconnecting!", act.user_id);
                
                // Notify server
                act.server.do_send(ServerMessage::Disconnect {
                    user_id: act.user_id.clone(),
                });
                
                // Stop actor
                ctx.stop();
                return;
            }
            
            ctx.ping(b"");
        });
    }
    
    /// Parse and handle incoming WebSocket message
    fn handle_text_message(&mut self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        debug!("Received message from {}: {}", self.user_id, text);
        
        // Try to parse the message
        match serde_json::from_str::<WebSocketMessage>(&text) {
            Ok(msg) => {
                // Forward to chat server
                self.server.do_send(ServerMessage::Message {
                    user_id: self.user_id.clone(),
                    msg,
                });
            }
            Err(e) => {
                error!("Failed to parse message from {}: {}", self.user_id, e);
                
                // Send error back to client
                let error_msg = WebSocketMessage::Error(ErrorMessage {
                    code: "PARSE_ERROR".to_string(),
                    message: format!("Invalid message format: {}", e),
                });
                
                if let Ok(json) = serde_json::to_string(&error_msg) {
                    ctx.text(json);
                }
            }
        }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;
    
    /// Method is called on actor start
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket session started for user {}", self.user_id);
        
        // Start heartbeat
        self.heartbeat(ctx);
        
        // Register with chat server
        self.server.do_send(ServerMessage::Connect {
            user_id: self.user_id.clone(),
            addr: ctx.address(),
        });
    }
    
    /// Method is called on actor stop
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        info!("WebSocket session stopping for user {}", self.user_id);
        
        // Notify server about disconnect
        self.server.do_send(ServerMessage::Disconnect {
            user_id: self.user_id.clone(),
        });
        
        Running::Stop
    }
}

/// Handle messages from chat server
impl Handler<SessionMessage> for ChatSession {
    type Result = ();
    
    fn handle(&mut self, msg: SessionMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Handle WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                error!("WebSocket error for user {}: {}", self.user_id, e);
                ctx.stop();
                return;
            }
        };
        
        match msg {
            ws::Message::Ping(msg) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            
            ws::Message::Pong(_) => {
                self.heartbeat = Instant::now();
            }
            
            ws::Message::Text(text) => {
                self.handle_text_message(text.to_string(), ctx);
            }
            
            ws::Message::Binary(bin) => {
                warn!("Binary messages not supported");
                ctx.binary(bin);
            }
            
            ws::Message::Close(reason) => {
                info!("WebSocket closing for user {}: {:?}", self.user_id, reason);
                ctx.close(reason);
                ctx.stop();
            }
            
            ws::Message::Continuation(_) => {
                warn!("Continuation frames not supported");
                ctx.stop();
            }
            
            ws::Message::Nop => {}
        }
    }
}