use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use log::{debug, info, warn};
use super::messages::*;
use super::chat_session::ChatSession;

/// Central chat server that manages all WebSocket connections
pub struct ChatServer {
    /// Map of user_id to their session address
    sessions: HashMap<String, Addr<ChatSession>>,
    
    /// Map of conversation_id to set of user_ids
    conversations: HashMap<String, HashSet<String>>,
    
    /// Map of user_id to their active conversations
    user_conversations: HashMap<String, HashSet<String>>,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            sessions: HashMap::new(),
            conversations: HashMap::new(),
            user_conversations: HashMap::new(),
        }
    }
    
    /// Add user to a conversation
    fn join_conversation(&mut self, user_id: &str, conversation_id: &str) {
        self.conversations
            .entry(conversation_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(user_id.to_string());
            
        self.user_conversations
            .entry(user_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(conversation_id.to_string());
    }
    
    /// Remove user from all conversations
    fn leave_all_conversations(&mut self, user_id: &str) {
        if let Some(conversations) = self.user_conversations.remove(user_id) {
            for conv_id in conversations {
                if let Some(users) = self.conversations.get_mut(&conv_id) {
                    users.remove(user_id);
                    if users.is_empty() {
                        self.conversations.remove(&conv_id);
                    }
                }
            }
        }
    }
    
    /// Send message to specific user
    fn send_to_user(&self, user_id: &str, message: WebSocketMessage) {
        if let Some(addr) = self.sessions.get(user_id) {
            let json = serde_json::to_string(&message).unwrap_or_default();
            addr.do_send(SessionMessage(json));
        }
    }
    
    /// Broadcast message to all users in a conversation
    fn broadcast_to_conversation(&self, conversation_id: &str, message: WebSocketMessage, exclude_user: Option<&str>) {
        if let Some(users) = self.conversations.get(conversation_id) {
            for user_id in users {
                if exclude_user.map_or(true, |excluded| excluded != user_id) {
                    self.send_to_user(user_id, message.clone());
                }
            }
        }
    }
    
    /// Notify users about online status change
    fn broadcast_online_status(&self, user_id: &str, is_online: bool) {
        let status_update = WebSocketMessage::OnlineStatus(OnlineStatusUpdate {
            user_id: user_id.to_string(),
            is_online,
            last_seen: chrono::Utc::now(),
        });
        
        // Send to all users who share conversations with this user
        if let Some(conversations) = self.user_conversations.get(user_id) {
            let mut notified_users = HashSet::new();
            
            for conv_id in conversations {
                if let Some(users) = self.conversations.get(conv_id) {
                    for other_user in users {
                        if other_user != user_id && !notified_users.contains(other_user) {
                            self.send_to_user(other_user, status_update.clone());
                            notified_users.insert(other_user.clone());
                        }
                    }
                }
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
    
    fn started(&mut self, _: &mut Context<Self>) {
        info!("Chat server started");
    }
}

/// Handle connection messages
impl Handler<ServerMessage> for ChatServer {
    type Result = ();
    
    fn handle(&mut self, msg: ServerMessage, _: &mut Context<Self>) {
        match msg {
            ServerMessage::Connect { user_id, addr } => {
                info!("User {} connected", user_id);
                
                // Store session
                self.sessions.insert(user_id.clone(), addr);
                
                // Send connection confirmation
                self.send_to_user(&user_id, WebSocketMessage::Connection(ConnectionStatus {
                    status: "connected".to_string(),
                    user_id: user_id.clone(),
                }));
                
                // Broadcast online status
                self.broadcast_online_status(&user_id, true);
            }
            
            ServerMessage::Disconnect { user_id } => {
                info!("User {} disconnected", user_id);
                
                // Remove session
                self.sessions.remove(&user_id);
                
                // Broadcast offline status
                self.broadcast_online_status(&user_id, false);
                
                // Clean up conversations
                self.leave_all_conversations(&user_id);
            }
            
            ServerMessage::Message { user_id, msg } => {
                debug!("Processing message from user {}", user_id);
                
                match &msg {
                    WebSocketMessage::Message(chat_msg) => {
                        // Join conversation if not already in it
                        self.join_conversation(&user_id, &chat_msg.conversation_id);
                        self.join_conversation(&chat_msg.recipient_id, &chat_msg.conversation_id);
                        
                        // Send to recipient
                        self.send_to_user(&chat_msg.recipient_id, msg.clone());
                    }
                    
                    WebSocketMessage::Typing(typing) => {
                        // Broadcast typing indicator to conversation
                        self.broadcast_to_conversation(
                            &typing.conversation_id,
                            msg.clone(),
                            Some(&user_id)
                        );
                    }
                    
                    _ => {
                        // Handle other message types
                        debug!("Unhandled message type from user {}", user_id);
                    }
                }
            }
            
            ServerMessage::BroadcastToConversation { conversation_id, msg, exclude_user } => {
                self.broadcast_to_conversation(
                    &conversation_id,
                    msg,
                    exclude_user.as_deref()
                );
            }
        }
    }
}

/// Message to send to a session
#[derive(Message)]
#[rtype(result = "()")]
pub struct SessionMessage(pub String);