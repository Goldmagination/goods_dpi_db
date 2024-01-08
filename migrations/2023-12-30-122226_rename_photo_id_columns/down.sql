-- This file should undo anything in `up.sql`
ALTER TABLE users RENAME COLUMN image_url TO photo_id;
ALTER TABLE professionals RENAME COLUMN image_url TO photo_id;
ALTER TABLE professional_profiles RENAME COLUMN image_url TO photo_id;
ALTER TABLE appointment_assignments RENAME COLUMN image_url TO photo_id;
ALTER TABLE message_assignments RENAME COLUMN image_url TO photo_id;
ALTER TABLE review_content_assignments RENAME COLUMN image_url TO photo_id;
