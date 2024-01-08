-- Your SQL goes here
ALTER TABLE users RENAME COLUMN photo_id TO image_url;
ALTER TABLE professionals RENAME COLUMN photo_id TO image_url;
ALTER TABLE professional_profiles RENAME COLUMN photo_id TO image_url;
ALTER TABLE appointment_assignments RENAME COLUMN photo_id TO image_url;
ALTER TABLE message_assignments RENAME COLUMN photo_id TO image_url;
ALTER TABLE review_content_assignments RENAME COLUMN photo_id TO image_url;
