-- Ensure the uuid-ossp extension is available
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Step 1: Add new UUID columns
ALTER TABLE users ADD COLUMN new_user_uid UUID;
ALTER TABLE chat ADD COLUMN new_user_uid UUID;
ALTER TABLE chat ADD COLUMN new_professional_profile_uid UUID;
ALTER TABLE message ADD COLUMN new_receiver_uid UUID;
ALTER TABLE message ADD COLUMN new_sender_uid UUID;
ALTER TABLE professional_profiles ADD COLUMN professional_profile_uid UUID;

-- Step 2: Generate UUIDs for professional_profiles if they don't exist
UPDATE professional_profiles SET professional_profile_uid = uuid_generate_v4() WHERE professional_profile_uid IS NULL;

-- Step 3: Ensure all user_uid values in users are valid UUIDs and generate new ones if necessary
UPDATE users SET user_uid = uuid_generate_v4() WHERE user_uid IS NULL OR user_uid !~* '^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$';

-- Step 4: Populate the new UUID column in users from the existing user_uid column
UPDATE users SET new_user_uid = user_uid::uuid;

-- Step 5: Populate the new UUID columns in chat and message tables using new_user_uid and professional_profile_uid
UPDATE chat SET new_user_uid = (SELECT new_user_uid FROM users WHERE users.id = chat.user_id);
UPDATE chat SET new_professional_profile_uid = (SELECT professional_profile_uid FROM professional_profiles WHERE professional_profiles.id = chat.professional_profile_id);

UPDATE message SET new_sender_uid = (SELECT new_user_uid FROM users WHERE users.id = message.sender_id);
UPDATE message SET new_receiver_uid = (SELECT professional_profile_uid FROM professional_profiles WHERE professional_profiles.id = message.receiver_id);

-- Step 6: Drop old integer and varchar columns
ALTER TABLE chat DROP COLUMN user_id;
ALTER TABLE chat DROP COLUMN professional_profile_id;
ALTER TABLE message DROP COLUMN sender_id;
ALTER TABLE message DROP COLUMN receiver_id;
ALTER TABLE users DROP COLUMN user_uid;

-- Step 7: Rename the new UUID columns to match the old names
ALTER TABLE chat RENAME COLUMN new_user_uid TO user_uid;
ALTER TABLE chat RENAME COLUMN new_professional_profile_uid TO professional_profile_uid;
ALTER TABLE message RENAME COLUMN new_sender_uid TO sender_uid;
ALTER TABLE message RENAME COLUMN new_receiver_uid TO receiver_uid;
ALTER TABLE users RENAME COLUMN new_user_uid TO user_uid;

-- Step 8: Set NOT NULL constraints if necessary
ALTER TABLE users ALTER COLUMN user_uid SET NOT NULL;
ALTER TABLE professional_profiles ALTER COLUMN professional_profile_uid SET NOT NULL;
ALTER TABLE chat ALTER COLUMN user_uid SET NOT NULL;
ALTER TABLE chat ALTER COLUMN professional_profile_uid SET NOT NULL;
ALTER TABLE message ALTER COLUMN sender_uid SET NOT NULL;
ALTER TABLE message ALTER COLUMN receiver_uid SET NOT NULL;
