-- Your SQL goes here
CREATE TABLE business_hours (
    id SERIAL PRIMARY KEY,
    professional_profile_id INTEGER NOT NULL REFERENCES professional_profiles(id),
    day_of_week INTEGER NOT NULL,
    opening_time TIME,
    closing_time TIME,
    is_available BOOLEAN NOT NULL
);
