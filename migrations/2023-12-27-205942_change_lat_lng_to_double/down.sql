ALTER TABLE addresses
ALTER COLUMN lat TYPE numeric(9,6) USING lat::numeric,
ALTER COLUMN lng TYPE numeric(9,6) USING lng::numeric;
