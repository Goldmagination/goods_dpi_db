-- This file should undo anything in `up.sql`
-- Drop `task_assignments` table
DROP TABLE IF EXISTS task_assignments;

-- Remove the new columns from the `task` table
ALTER TABLE task
DROP COLUMN title,
DROP COLUMN min_price,
DROP COLUMN max_price,
DROP COLUMN is_flexible_timing,
DROP COLUMN scheduled_date,
DROP COLUMN scheduled_time,
DROP COLUMN category_id;

-- Rename `task` back to `tasks`
ALTER TABLE task RENAME TO tasks;

-- Recreate the `tasks_subcategories` table
CREATE TABLE tasks_subcategories (
    id SERIAL PRIMARY KEY,
    task_id INT4 NOT NULL REFERENCES tasks(id),
    subcategory_id INT4 NOT NULL
);
