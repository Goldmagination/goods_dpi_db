-- Your SQL goes here
ALTER TABLE review
ADD COLUMN published_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;
