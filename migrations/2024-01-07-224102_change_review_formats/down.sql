-- This file should undo anything in `up.sql`
ALTER TABLE review
ALTER COLUMN rate TYPE Numeric;
ALTER TABLE review
ALTER COLUMN rate DROP NOT NULL;