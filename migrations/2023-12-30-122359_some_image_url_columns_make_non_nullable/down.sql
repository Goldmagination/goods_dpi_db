-- This file should undo anything in `up.sql`
ALTER TABLE appointment_assignments ALTER COLUMN image_url DROP NOT NULL;
ALTER TABLE message_assignments ALTER COLUMN image_url DROP NOT NULL;
ALTER TABLE review_content_assignments ALTER COLUMN image_url DROP NOT NULL;