-- Ensure the uuid-ossp extension is available
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Step 1: Add new UID columns
ALTER TABLE users ADD COLUMN new_user_uid VARCHAR(255);
ALTER TABLE professionals ADD COLUMN new_user_uid VARCHAR(255);
ALTER TABLE chat ADD COLUMN new_user_uid VARCHAR(255);
ALTER TABLE chat ADD COLUMN new_professional_profile_uid VARCHAR(255);
ALTER TABLE message ADD COLUMN new_receiver_uid VARCHAR(255);
ALTER TABLE message ADD COLUMN new_sender_uid VARCHAR(255);
ALTER TABLE professional_profiles ADD COLUMN new_professional_profile_uid VARCHAR(255);

-- Step 2: Generate new UIDs where old ids are null
UPDATE users SET new_user_uid = gen_random_uuid()::text WHERE user_uid IS NULL;
UPDATE professionals SET new_user_uid = gen_random_uuid()::text WHERE user_uid IS NULL;
UPDATE professional_profiles SET new_professional_profile_uid = gen_random_uuid()::text WHERE professional_profile_uid IS NULL;

-- Step 3: Transfer existing non-null ids as new UIDs
UPDATE users SET new_user_uid = user_uid WHERE user_uid IS NOT NULL;
UPDATE professionals SET new_user_uid = user_uid WHERE user_uid IS NOT NULL;
UPDATE professional_profiles SET new_professional_profile_uid = professional_profile_uid WHERE professional_profile_uid IS NOT NULL;

-- Step 4: Populate the new UID columns
UPDATE chat SET new_user_uid = (SELECT new_user_uid FROM users WHERE users.id = chat.user_id);
UPDATE chat SET new_professional_profile_uid = (SELECT new_professional_profile_uid FROM professional_profiles WHERE professional_profiles.id = chat.professional_profile_id);
UPDATE message SET new_sender_uid = (SELECT new_user_uid FROM users WHERE users.id = message.sender_id);
UPDATE message SET new_receiver_uid = (SELECT new_professional_profile_uid FROM professional_profiles WHERE professional_profiles.id = message.receiver_id);

-- Step 5: Drop old integer and varchar columns
ALTER TABLE chat DROP COLUMN user_id;
ALTER TABLE chat DROP COLUMN professional_profile_id;
ALTER TABLE message DROP COLUMN sender_id;
ALTER TABLE message DROP COLUMN receiver_id;
ALTER TABLE users DROP COLUMN user_uid;
ALTER TABLE professionals DROP COLUMN user_uid;
ALTER TABLE professional_profiles DROP COLUMN professional_profile_uid;

-- Step 6: Rename the new UID columns to match the old names
ALTER TABLE chat RENAME COLUMN new_user_uid TO user_uid;
ALTER TABLE chat RENAME COLUMN new_professional_profile_uid TO professional_profile_uid;
ALTER TABLE message RENAME COLUMN new_sender_uid TO sender_uid;
ALTER TABLE message RENAME COLUMN new_receiver_uid TO receiver_uid;
ALTER TABLE users RENAME COLUMN new_user_uid TO user_uid;
ALTER TABLE professionals RENAME COLUMN new_user_uid TO user_uid;
ALTER TABLE professional_profiles RENAME COLUMN new_professional_profile_uid TO professional_profile_uid;

-- Step 7: Set NOT NULL constraints if necessary
ALTER TABLE users ALTER COLUMN user_uid SET NOT NULL;
ALTER TABLE professionals ALTER COLUMN user_uid SET NOT NULL;
ALTER TABLE professional_profiles ALTER COLUMN professional_profile_uid SET NOT NULL;
ALTER TABLE chat ALTER COLUMN user_uid SET NOT NULL;
ALTER TABLE chat ALTER COLUMN professional_profile_uid SET NOT NULL;
ALTER TABLE message ALTER COLUMN sender_uid SET NOT NULL;
ALTER TABLE message ALTER COLUMN receiver_uid SET NOT NULL;
