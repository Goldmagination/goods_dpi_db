-- Your SQL goes here
ALTER TABLE review
ALTER COLUMN rate TYPE double precision;
ALTER TABLE review
ALTER COLUMN rate SET NOT NULL;