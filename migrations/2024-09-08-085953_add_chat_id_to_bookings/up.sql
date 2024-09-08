-- Your SQL goes here
ALTER TABLE bookings
ADD COLUMN chat_id INTEGER;

ALTER TABLE bookings
ADD COLUMN service_offering_name VARCHAR(255);

UPDATE bookings
SET chat_id = 1
WHERE chat_id IS NULL;


ALTER TABLE bookings
ALTER COLUMN chat_id SET NOT NULL;


ALTER TABLE bookings
ADD CONSTRAINT fk_chat_id
FOREIGN KEY (chat_id)
REFERENCES chat(id);


ALTER TABLE bookings
ADD COLUMN creation_time TIMESTAMP WITH TIME ZONE;

UPDATE bookings
SET creation_time = NOW() AT TIME ZONE 'UTC'
WHERE creation_time IS NULL;


ALTER TABLE bookings
ALTER COLUMN creation_time SET NOT NULL;
