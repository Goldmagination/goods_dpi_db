-- This file should undo anything in `up.sql`

ALTER TABLE bookings
DROP CONSTRAINT fk_chat_id;

-- Drop the chat_id column
ALTER TABLE bookings
DROP COLUMN chat_id;

ALTER TABLE bookings
DROP COLUMN service_offering_name;

-- Drop the creation_time column
ALTER TABLE bookings
DROP COLUMN creation_time;
