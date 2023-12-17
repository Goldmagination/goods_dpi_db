CREATE TABLE review (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    professional_profile_id INTEGER NOT NULL REFERENCES professional_profiles(id),
    message TEXT NOT NULL,
    rate NUMERIC CHECK (rate >= 0.0 AND rate <= 5.0)
);

CREATE TABLE review_content_assignments (
    id SERIAL PRIMARY KEY,
    review_id INTEGER NOT NULL REFERENCES review(id),
    photo_id INTEGER NOT NULL -- Replace with the appropriate reference if you have a photos table
);

