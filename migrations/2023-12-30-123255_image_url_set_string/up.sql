-- Your SQL goes here
ALTER TABLE users ALTER COLUMN image_url TYPE VARCHAR(255);
ALTER TABLE professionals ALTER COLUMN image_url TYPE VARCHAR(255);
ALTER TABLE professional_profiles ALTER COLUMN image_url TYPE VARCHAR(255);
ALTER TABLE appointment_assignments ALTER COLUMN image_url TYPE VARCHAR(255);
ALTER TABLE message_assignments ALTER COLUMN image_url TYPE VARCHAR(255);
ALTER TABLE review_content_assignments ALTER COLUMN image_url TYPE VARCHAR(255);