-- Your SQL goes here
-- Rename `tasks` table to `task`
ALTER TABLE tasks RENAME TO task;

-- Add new columns to the `task` table
ALTER TABLE task
ADD COLUMN title VARCHAR NOT NULL,
ADD COLUMN min_price DOUBLE PRECISION,
ADD COLUMN max_price DOUBLE PRECISION,
ADD COLUMN is_flexible_timing BOOLEAN NOT NULL DEFAULT true,
ADD COLUMN scheduled_date DATE,
ADD COLUMN scheduled_time TIME,
ADD COLUMN category_id INT4;

-- Drop the `tasks_subcategories` table
DROP TABLE IF EXISTS tasks_subcategories;

-- Create the `task_assignments` table
CREATE TABLE task_assignments (
    id SERIAL PRIMARY KEY,
    task_id INT4 NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    image_url VARCHAR NOT NULL
);
