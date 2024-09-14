-- This file should undo anything in `up.sql`
ALTER TABLE message
ALTER COLUMN text SET NOT NULL;
