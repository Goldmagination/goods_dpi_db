# WebSocket Integration for Goods Backend

## Overview
WebSocket support has been successfully integrated into the Goods backend, enabling real-time chat functionality between users.

## Features Implemented

✅ **Real-time Messaging**
- Direct messages between users
- Conversation-based message routing
- Message broadcasting to all participants

✅ **Connection Management**
- User authentication (JWT ready)
- Automatic heartbeat/ping-pong
- Connection/disconnection tracking

✅ **Real-time Features**
- Typing indicators
- Online/offline status tracking
- Read receipts support

✅ **Error Handling**
- Invalid message format handling
- Connection timeout management
- Graceful disconnection

## WebSocket Endpoints

### Main WebSocket Endpoint
```
ws://localhost:8080/ws/chat/{user_id}
```
Optional query parameter: `?token=JWT_TOKEN` (for authentication)

### Health Check
```
GET /ws/health
```

## Message Formats

### Send a Chat Message
```json
{
  "type": "message",
  "data": {
    "id": "unique-message-id",
    "conversation_id": "conv-123",
    "sender_id": "user1",
    "recipient_id": "user2",
    "text": "Hello!",
    "timestamp": "2024-01-01T00:00:00Z",
    "is_read": false
  }
}
```

### Typing Indicator
```json
{
  "type": "typing",
  "data": {
    "conversation_id": "conv-123",
    "user_id": "user1",
    "is_typing": true
  }
}
```

### Online Status
```json
{
  "type": "online_status",
  "data": {
    "user_id": "user1",
    "is_online": true,
    "last_seen": "2024-01-01T00:00:00Z"
  }
}
```

## Testing the WebSocket

### 1. Start the Backend
```bash
cd /Users/kuban/Goods/goods_backend_repo
cargo run
```

### 2. Test with the provided script
```bash
./test_websocket.sh
```

### 3. Test with curl (health check)
```bash
curl http://localhost:8080/health
curl http://localhost:8080/ws/health
```

## Flutter App Integration

The Flutter app is already configured to connect to the WebSocket. When you run the backend:

1. **Development**: The app will connect to `ws://192.168.2.37:8080/ws/chat/{userId}`
2. **Production**: Update to use `wss://goods-backend.fly.dev/ws/chat/{userId}`

## Database Integration (Optional)

To persist chat messages to your PostgreSQL database, you can add this to the message handler in `chat_server.rs`:

```rust
// In ServerMessage::Message handler
use crate::dal::chat_db;
use crate::models::chat_aggregate::message::NewMessage;

// Save message to database
let new_message = NewMessage {
    id: chat_msg.id.clone(),
    conversation_id: chat_msg.conversation_id.clone(),
    sender_id: chat_msg.sender_id.clone(),
    text: chat_msg.text.clone(),
    timestamp: chat_msg.timestamp,
    is_read: chat_msg.is_read,
};

// Use your existing DAL to save
chat_db::create_message(&db_pool, new_message)?;
```

## Environment Variables

Add these to your `.env` file:
```env
JWT_SECRET=your-secret-key-here
HOST=0.0.0.0
PORT=8080
```

## Deployment

### For Fly.io

1. Ensure your `fly.toml` includes:
```toml
[services]
  internal_port = 8080
  protocol = "tcp"
  
  [[services.ports]]
    handlers = ["http", "tls"]
    port = "443"
```

2. Deploy:
```bash
fly deploy
```

### For Docker

Add to your Dockerfile:
```dockerfile
EXPOSE 8080
ENV HOST=0.0.0.0
ENV PORT=8080
```

## Monitoring

Check WebSocket connections and messages:
```bash
# Development
cargo run 2>&1 | grep -E "WebSocket|Chat"

# Production (Fly.io)
fly logs --app goods-backend
```

## Troubleshooting

### Connection Refused
- Ensure backend is running on port 8080
- Check firewall settings
- Verify CORS configuration

### Messages Not Delivering
- Check user IDs match exactly
- Verify conversation_id is consistent
- Look for errors in server logs

### High Memory Usage
- Implement message cleanup for old conversations
- Add connection limits per user
- Consider Redis for session management

## Next Steps

1. **Add JWT Authentication**: Uncomment JWT validation in `websocket_handler.rs`
2. **Persist Messages**: Integrate with your existing chat DAL
3. **Add File Uploads**: Extend message format to support attachments
4. **Implement Groups**: Add group chat support
5. **Add Push Notifications**: Notify offline users

## Files Modified

- `Cargo.toml` - Added WebSocket dependencies
- `src/main.rs` - Integrated WebSocket server
- `src/websocket/` - New WebSocket module
  - `mod.rs` - Module definition
  - `chat_server.rs` - Main chat server actor
  - `chat_session.rs` - Individual user sessions
  - `messages.rs` - Message types
- `src/handlers/websocket_handler.rs` - HTTP to WebSocket upgrade
- `src/middleware/auth.rs` - JWT validation (ready to use)