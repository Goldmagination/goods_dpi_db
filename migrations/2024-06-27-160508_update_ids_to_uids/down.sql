-- Step 1: Add back the old integer and varchar columns
ALTER TABLE users ADD COLUMN old_user_uid VARCHAR(255);
ALTER TABLE professionals ADD COLUMN old_user_uid VARCHAR(255);
ALTER TABLE chat ADD COLUMN old_user_id INT;
ALTER TABLE chat ADD COLUMN old_professional_profile_id INT;
ALTER TABLE message ADD COLUMN old_sender_id INT;
ALTER TABLE message ADD COLUMN old_receiver_id INT;
ALTER TABLE professional_profiles ADD COLUMN old_professional_profile_uid VARCHAR(255);

-- Step 2: Transfer data from new UID columns back to old columns
UPDATE users SET old_user_uid = user_uid;
UPDATE professionals SET old_user_uid = user_uid;
UPDATE professional_profiles SET old_professional_profile_uid = professional_profile_uid;

-- Populate old IDs back from the chat and message tables
UPDATE chat SET old_user_id = (SELECT id FROM users WHERE users.user_uid = chat.user_uid);
UPDATE chat SET old_professional_profile_id = (SELECT id FROM professional_profiles WHERE professional_profiles.professional_profile_uid = chat.professional_profile_uid);
UPDATE message SET old_sender_id = (SELECT id FROM users WHERE users.user_uid = message.sender_uid);
UPDATE message SET old_receiver_id = (SELECT id FROM professional_profiles WHERE professional_profiles.professional_profile_uid = message.receiver_uid);

-- Step 3: Drop the new UID columns
ALTER TABLE users DROP COLUMN user_uid;
ALTER TABLE professionals DROP COLUMN user_uid;
ALTER TABLE professional_profiles DROP COLUMN professional_profile_uid;
ALTER TABLE chat DROP COLUMN user_uid;
ALTER TABLE chat DROP COLUMN professional_profile_uid;
ALTER TABLE message DROP COLUMN sender_uid;
ALTER TABLE message DROP COLUMN receiver_uid;

-- Step 4: Rename old columns back to their original names
ALTER TABLE users RENAME COLUMN old_user_uid TO user_uid;
ALTER TABLE professionals RENAME COLUMN old_user_uid TO user_uid;
ALTER TABLE professional_profiles RENAME COLUMN old_professional_profile_uid TO professional_profile_uid;
ALTER TABLE chat RENAME COLUMN old_user_id TO user_id;
ALTER TABLE chat RENAME COLUMN old_professional_profile_id TO professional_profile_id;
ALTER TABLE message RENAME COLUMN old_sender_id TO sender_id;
ALTER TABLE message RENAME COLUMN old_receiver_id TO receiver_id;

-- Step 5: Drop the NOT NULL constraints if necessary
ALTER TABLE users ALTER COLUMN user_uid DROP NOT NULL;
ALTER TABLE professionals ALTER COLUMN user_uid DROP NOT NULL;
ALTER TABLE professional_profiles ALTER COLUMN professional_profile_uid DROP NOT NULL;
ALTER TABLE chat ALTER COLUMN user_id DROP NOT NULL;
ALTER TABLE chat ALTER COLUMN professional_profile_id DROP NOT NULL;
ALTER TABLE message ALTER COLUMN sender_id DROP NOT NULL;
ALTER TABLE message ALTER COLUMN receiver_id DROP NOT NULL;

-- Step 6: Drop the uuid-ossp extension if it's no longer needed
DROP EXTENSION IF EXISTS "uuid-ossp";
