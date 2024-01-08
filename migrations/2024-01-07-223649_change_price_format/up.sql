-- Your SQL goes here
ALTER TABLE service_offerings
ALTER COLUMN price TYPE double precision;
ALTER TABLE service_offerings
ALTER COLUMN subcategory_name SET NOT NULL;