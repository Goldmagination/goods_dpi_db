-- Your SQL goes here
ALTER TABLE professional_profiles
ADD COLUMN professional_name VARCHAR;
UPDATE professional_profiles
SET professional_name = 'John Doe'
WHERE id = 1;
UPDATE professional_profiles
SET professional_name = 'Allen Sinner'
WHERE id = 8;
UPDATE professional_profiles
SET professional_name = 'Ben Stigler'
WHERE id = 9;
UPDATE professional_profiles
SET professional_name = 'Don Landing'
WHERE id = 10;
ALTER TABLE professional_profiles
ALTER COLUMN professional_name SET NOT NULL;
