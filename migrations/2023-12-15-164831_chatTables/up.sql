CREATE TABLE chat (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    professional_id INTEGER NOT NULL REFERENCES professionals(id),
    title VARCHAR NOT NULL,
    last_message_time TIMESTAMP NOT NULL
);

CREATE TABLE message (
    id SERIAL PRIMARY KEY,
    chat_id INTEGER NOT NULL REFERENCES chat(id),
    sender_id INTEGER NOT NULL,
    text TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    is_read BOOLEAN NOT NULL
);

CREATE TABLE message_assignments (
    id SERIAL PRIMARY KEY,
    message_id INTEGER NOT NULL REFERENCES message(id),
    photo_id INTEGER NOT NULL
);
