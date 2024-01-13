-- This file should undo anything in `up.sql`
ALTER TABLE tasks
RENAME TO orders;

ALTER TABLE tasks_subcategories
RENAME TO order_subcategories;

ALTER TABLE order_subcategories
RENAME COLUMN task_id TO order_id;

ALTER TABLE orders
DROP COLUMN price;

ALTER TABLE orders 
ALTER COLUMN address_id SET NOT NULL;