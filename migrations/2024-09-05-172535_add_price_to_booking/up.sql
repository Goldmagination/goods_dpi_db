-- Your SQL goes here
ALTER TABLE bookings
ADD COLUMN offering_price FLOAT8 NOT NULL DEFAULT 0.0;
