-- This file should undo anything in `up.sql`
ALTER TABLE chat
DROP CONSTRAINT if exists chat_professional_profile_id_fkey;

ALTER TABLE chat
RENAME COLUMN professional_profile_id TO professional_id;

ALTER TABLE chat
ADD CONSTRAINT chat_professional_id_fkey
FOREIGN KEY (professional_id)
REFERENCES professionals (id);

ALTER TABLE message
DROP COLUMN IF EXISTS receiver_id;