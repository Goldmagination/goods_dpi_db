-- Create status table
CREATE TABLE appointment_status (
    id SERIAL PRIMARY KEY,
    description VARCHAR(255) NOT NULL
);

-- Rename message to description in appointments table
ALTER TABLE appointments
    RENAME COLUMN message TO description;

-- Drop existing foreign key constraint on professional_profile_id if it exists
ALTER TABLE appointments
    DROP CONSTRAINT IF EXISTS appointments_professional_profile_id_fkey;

-- Drop existing foreign key constraint on customer_id if it exists
ALTER TABLE appointments
    DROP CONSTRAINT IF EXISTS appointments_customer_id_fkey;

-- Rename professional_profile_id to professional_profile_uid and change its type to VARCHAR(255)
ALTER TABLE appointments
    RENAME COLUMN professional_profile_id TO professional_profile_uid;
ALTER TABLE appointments
    ALTER COLUMN professional_profile_uid TYPE VARCHAR(255);

-- Ensure professional_profile_uid in professional_profiles is unique
ALTER TABLE professional_profiles
    ADD CONSTRAINT unique_professional_profile_uid UNIQUE (professional_profile_uid);

-- Add foreign key constraint for professional_profile_uid to professional_profiles table
ALTER TABLE appointments
    ADD CONSTRAINT fk_professional_profile_uid FOREIGN KEY (professional_profile_uid) REFERENCES professional_profiles(professional_profile_uid);

-- Rename customer_id to customer_uid and change its type to VARCHAR(255)
ALTER TABLE appointments
    RENAME COLUMN customer_id TO customer_uid;
ALTER TABLE appointments
    ALTER COLUMN customer_uid TYPE VARCHAR(255);

-- Ensure user_uid in users is unique
ALTER TABLE users
    ADD CONSTRAINT unique_user_uid UNIQUE (user_uid);

-- Add foreign key constraint for customer_uid to users table
ALTER TABLE appointments
    ADD CONSTRAINT fk_customer_uid FOREIGN KEY (customer_uid) REFERENCES users(user_uid);

-- Add end_time column to appointments table
ALTER TABLE appointments
    ADD COLUMN end_time TIMESTAMPTZ;

-- Add service_offering_id to appointments table and link to service_offerings table
ALTER TABLE appointments
    ADD COLUMN service_offering_id INT4,
    ADD CONSTRAINT fk_offering FOREIGN KEY (service_offering_id) REFERENCES service_offerings(id);

-- Change date_time to UTC (TIMESTAMPTZ) and make it nullable
ALTER TABLE appointments
    ALTER COLUMN date_time TYPE TIMESTAMPTZ,
    ALTER COLUMN date_time DROP NOT NULL;
