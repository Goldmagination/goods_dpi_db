-- Your SQL goes here
ALTER TABLE orders
RENAME TO tasks;

ALTER TABLE order_subcategories
RENAME TO tasks_subcategories;

ALTER TABLE tasks_subcategories
RENAME COLUMN order_id TO task_id;

ALTER TABLE tasks
ADD COLUMN price INTEGER;

ALTER TABLE tasks 
ALTER COLUMN address_id DROP NOT NULL;
