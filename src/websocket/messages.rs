use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use actix::prelude::*;

/// Message types that match Flutter frontend expectations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    #[serde(rename = "message")]
    Message(ChatMessage),
    
    #[serde(rename = "typing")]
    Typing(TypingIndicator),
    
    #[serde(rename = "online_status")]
    OnlineStatus(OnlineStatusUpdate),
    
    #[serde(rename = "error")]
    Error(ErrorMessage),
    
    #[serde(rename = "connection")]
    Connection(ConnectionStatus),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub is_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingIndicator {
    pub conversation_id: String,
    pub user_id: String,
    pub is_typing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineStatusUpdate {
    pub user_id: String,
    pub is_online: bool,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub status: String, // "connected" | "disconnected" | "reconnecting"
    pub user_id: String,
}

/// Internal server messages for managing WebSocket connections
#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum ServerMessage {
    Connect { user_id: String, addr: Addr<super::chat_session::ChatSession> },
    Disconnect { user_id: String },
    Message { user_id: String, msg: WebSocketMessage },
    BroadcastToConversation { conversation_id: String, msg: WebSocketMessage, exclude_user: Option<String> },
}