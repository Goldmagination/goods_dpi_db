ALTER TABLE addresses
ALTER COLUMN lat TYPE double precision USING lat::double precision,
ALTER COLUMN lng TYPE double precision USING lng::double precision;
