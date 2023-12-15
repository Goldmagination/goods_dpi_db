CREATE TABLE addresses (
    id SERIAL PRIMARY KEY,
    street VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    state VARCHAR NOT NULL,
    zip VARCHAR NOT NULL
);

CREATE TABLE address_assignments (
    id SERIAL PRIMARY KEY,
    professional_profile_id INTEGER NOT NULL REFERENCES professional_profiles(id),
    address_id INTEGER NOT NULL REFERENCES addresses(id)
);

