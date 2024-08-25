-- Your SQL goes here
-- Rename appointments table to bookings
ALTER TABLE appointments RENAME TO bookings;
ALTER TABLE appointment_assignments RENAME TO booking_assignments;
ALTER TABLE appointment_status RENAME TO booking_status;
