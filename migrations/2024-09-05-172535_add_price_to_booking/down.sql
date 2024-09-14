-- This file should undo anything in `up.sql`
ALTER TABLE bookings
DROP COLUMN offering_price;
