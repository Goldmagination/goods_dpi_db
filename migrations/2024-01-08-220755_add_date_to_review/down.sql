-- This file should undo anything in `up.sql`
ALTER TABLE review
DROP COLUMN published_at;
