-- Your SQL goes here
ALTER TABLE professional_profiles
ADD COLUMN remote_available BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE professional_profiles
ALTER COLUMN delivery_enabled SET NOT NULL;