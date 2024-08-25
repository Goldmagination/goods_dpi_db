-- This file should undo anything in `up.sql`
ALTER TABLE bookings RENAME TO appointments;
ALTER TABLE booking_assignments RENAME TO appointment_assignments;
ALTER TABLE booking_status RENAME TO appointment_status;
