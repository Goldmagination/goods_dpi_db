-- This file should undo anything in `up.sql`
ALTER TABLE users ALTER COLUMN image_url TYPE INTEGER USING image_url::integer;
ALTER TABLE professionals ALTER COLUMN image_url TYPE INTEGER USING image_url::integer;
ALTER TABLE professional_profiles ALTER COLUMN image_url TYPE INTEGER USING image_url::integer;
ALTER TABLE appointment_assignments ALTER COLUMN image_url TYPE INTEGER USING image_url::integer;
ALTER TABLE message_assignments ALTER COLUMN image_url TYPE INTEGER USING image_url::integer;
ALTER TABLE review_content_assignments ALTER COLUMN image_url TYPE INTEGER USING image_url::integer;
