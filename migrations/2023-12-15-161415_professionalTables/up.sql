CREATE TABLE professionals (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    user_uid VARCHAR NOT NULL
);

CREATE TABLE professional_profiles (
    id SERIAL PRIMARY KEY,
    professional_id INTEGER NOT NULL REFERENCES professionals(id),
    category_id INTEGER NOT NULL,
    credentials TEXT,
    delivery_enabled BOOLEAN
);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT
);

CREATE TABLE subcategories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    category_id INTEGER NOT NULL REFERENCES categories(id)
);

CREATE TABLE service_offerings (
    id SERIAL PRIMARY KEY,
    professional_profile_id INTEGER NOT NULL REFERENCES professional_profiles(id),
    subcategory_id INTEGER NOT NULL REFERENCES subcategories(id),
    price NUMERIC NOT NULL
);
