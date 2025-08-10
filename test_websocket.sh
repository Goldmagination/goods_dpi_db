#!/bin/bash

# Test WebSocket connection script
echo "Testing WebSocket connection..."

# Install wscat if not installed
if ! command -v wscat &> /dev/null; then
    echo "Installing wscat..."
    npm install -g wscat
fi

# Start the backend in background
echo "Starting backend server..."
cargo run &
SERVER_PID=$!

# Wait for server to start
sleep 5

# Test WebSocket connection
echo "Testing WebSocket connection to ws://localhost:8080/ws/chat/testuser123..."
echo "Send a message in JSON format like:"
echo '{"type":"message","data":{"id":"msg1","conversation_id":"conv1","sender_id":"testuser123","recipient_id":"user456","text":"Hello!","timestamp":"2024-01-01T00:00:00Z","is_read":false}}'
echo ""
echo "Connecting..."

# Connect to WebSocket (without JWT for testing)
wscat -c "ws://localhost:8080/ws/chat/testuser123"

# Clean up
kill $SERVER_PID