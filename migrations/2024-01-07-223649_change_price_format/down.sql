-- This file should undo anything in `up.sql`
ALTER TABLE service_offerings
ALTER COLUMN price TYPE Numeric;
ALTER TABLE service_offerings
ALTER COLUMN subcategory_name DROP NOT NULL;
