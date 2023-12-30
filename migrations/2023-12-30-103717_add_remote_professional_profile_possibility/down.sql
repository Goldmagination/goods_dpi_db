-- This file should undo anything in `up.sql`
ALTER TABLE professional_profiles
DROP COLUMN remote_available;
ALTER TABLE professional_profiles
ALTER COLUMN delivery_enabled DROP NOT NULL;