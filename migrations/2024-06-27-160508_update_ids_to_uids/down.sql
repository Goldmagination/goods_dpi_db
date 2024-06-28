ALTER TABLE chat ADD COLUMN user_id INTEGER;
ALTER TABLE chat ADD COLUMN professional_profile_id INTEGER;
ALTER TABLE message ADD COLUMN sender_id INTEGER;
ALTER TABLE message ADD COLUMN receiver_id INTEGER;
ALTER TABLE users ADD COLUMN user_uid VARCHAR;

UPDATE chat SET user_id = (SELECT id FROM users WHERE users.user_uid = chat.user_uid::uuid);
UPDATE chat SET professional_profile_id = (SELECT id FROM professional_profiles WHERE professional_profiles.professional_profile_uid = chat.professional_profile_uid::uuid);

UPDATE message SET sender_id = (SELECT id FROM users WHERE users.user_uid = message.sender_uid::uuid);
UPDATE message SET receiver_id = (SELECT id FROM professional_profiles WHERE professional_profiles.professional_profile_uid = message.receiver_uid::uuid);

UPDATE users SET user_uid = new_user_uid::varchar WHERE user_uid IS NULL;

ALTER TABLE chat DROP COLUMN user_uid;
ALTER TABLE chat DROP COLUMN professional_profile_uid;
ALTER TABLE message DROP COLUMN sender_uid;
ALTER TABLE message DROP COLUMN receiver_uid;
ALTER TABLE users DROP COLUMN user_uid;

ALTER TABLE chat RENAME COLUMN new_user_uid TO user_id;
ALTER TABLE chat RENAME COLUMN new_professional_profile_uid TO professional_profile_id;
ALTER TABLE message RENAME COLUMN new_sender_uid TO sender_id;
ALTER TABLE message RENAME COLUMN new_receiver_uid TO receiver_id;

ALTER TABLE users ALTER COLUMN new_user_uid DROP NOT NULL;
ALTER TABLE professional_profiles ALTER COLUMN professional_profile_uid DROP NOT NULL;
