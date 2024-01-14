-- Your SQL goes here
ALTER TABLE chat
RENAME COLUMN professional_id TO professional_profile_id;

ALTER TABLE chat
DROP CONSTRAINT if exists chat_professional_id_fkey;

ALTER TABLE chat
ADD CONSTRAINT chat_professional_profile_id_fkey
FOREIGN KEY (professional_profile_id)
REFERENCES professional_profiles (id);

ALTER TABLE message
ADD COLUMN receiver_id INTEGER NOT NULL;
