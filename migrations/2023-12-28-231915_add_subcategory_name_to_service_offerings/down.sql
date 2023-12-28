-- This file should undo anything in `up.sql`
ALTER TABLE service_offerings
DROP COLUMN subcategory_name;
