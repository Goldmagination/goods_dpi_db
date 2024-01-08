-- This file should undo anything in `up.sql`
ALTER TABLE professional_profiles
ALTER COLUMN average_rating TYPE Numeric;
