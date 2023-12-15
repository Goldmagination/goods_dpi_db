CREATE TABLE appointments (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES users(id),
    professional_profile_id INTEGER NOT NULL REFERENCES professional_profiles(id),
    date_time TIMESTAMP NOT NULL,
    status INTEGER NOT NULL,
    message TEXT,
    category_id INTEGER NOT NULL
);
CREATE TABLE appointment_assignments (
    id SERIAL PRIMARY KEY,
    appointment_id INTEGER NOT NULL REFERENCES appointments(id),
    photo_id INTEGER
);


