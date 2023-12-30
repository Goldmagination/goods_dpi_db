-- Your SQL goes here
ALTER TABLE appointment_assignments ALTER COLUMN image_url SET NOT NULL;
ALTER TABLE message_assignments ALTER COLUMN image_url SET NOT NULL;
ALTER TABLE review_content_assignments ALTER COLUMN image_url SET NOT NULL;