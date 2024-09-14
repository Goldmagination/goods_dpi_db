-- Drop foreign key constraint and column service_offering_id
ALTER TABLE appointments
    DROP CONSTRAINT IF EXISTS fk_offering,
    DROP COLUMN service_offering_id;

-- Remove end_time column
ALTER TABLE appointments
    DROP COLUMN end_time;

-- Revert date_time to non-nullable and change back to TIMESTAMP (without time zone)
ALTER TABLE appointments
    ALTER COLUMN date_time TYPE TIMESTAMP,
    ALTER COLUMN date_time SET NOT NULL;

-- Drop foreign key constraints on professional_profile_uid and customer_uid
ALTER TABLE appointments
    DROP CONSTRAINT IF EXISTS fk_professional_profile_uid,
    DROP CONSTRAINT IF EXISTS fk_customer_uid;

-- Remove unique constraints on professional_profile_uid and user_uid
ALTER TABLE professional_profiles
    DROP CONSTRAINT IF EXISTS unique_professional_profile_uid;

ALTER TABLE users
    DROP CONSTRAINT IF EXISTS unique_user_uid;

-- Rename professional_profile_uid back to professional_profile_id and change type to INT4
ALTER TABLE appointments
    RENAME COLUMN professional_profile_uid TO professional_profile_id;
ALTER TABLE appointments
    ALTER COLUMN professional_profile_id TYPE INT4 USING professional_profile_id::int;

-- Rename customer_uid back to customer_id and change type to INT4
ALTER TABLE appointments
    RENAME COLUMN customer_uid TO customer_id;
ALTER TABLE appointments
    ALTER COLUMN customer_id TYPE INT4 USING customer_id::int;

-- Re-add the foreign key constraint on professional_profile_id
ALTER TABLE appointments
    ADD CONSTRAINT appointments_professional_profile_id_fkey FOREIGN KEY (professional_profile_id) REFERENCES professional_profiles(id);

-- Rename description back to message
ALTER TABLE appointments
    RENAME COLUMN description TO message;

-- Drop the appointment_status table
DROP TABLE IF EXISTS appointment_status;
