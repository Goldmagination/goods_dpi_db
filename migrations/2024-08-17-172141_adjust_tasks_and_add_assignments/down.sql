-- Drop the new foreign key constraint
ALTER TABLE task DROP CONSTRAINT IF EXISTS task_user_uid_fkey;

-- Drop `task_assignments` table
DROP TABLE IF EXISTS task_assignments;

-- Remove the new columns from the `task` table
ALTER TABLE task DROP COLUMN title;
ALTER TABLE task DROP COLUMN min_price;
ALTER TABLE task DROP COLUMN max_price;
ALTER TABLE task DROP COLUMN is_flexible_timing;
ALTER TABLE task DROP COLUMN scheduled_date;
ALTER TABLE task DROP COLUMN scheduled_time;
ALTER TABLE task DROP COLUMN category_id;

-- Rename user_uid back to user_id
ALTER TABLE task RENAME COLUMN user_uid TO user_id;

-- Change user_id type back to INT4
ALTER TABLE task ALTER COLUMN user_id TYPE INT4 USING user_id::integer;

-- Change creation_time back to TIMESTAMP
ALTER TABLE task ALTER COLUMN creation_time TYPE TIMESTAMP USING creation_time AT TIME ZONE 'UTC';

-- Rename `task` back to `tasks`
ALTER TABLE task RENAME TO tasks;

-- Recreate the `tasks_subcategories` table
CREATE TABLE tasks_subcategories (
    id SERIAL PRIMARY KEY,
    task_id INT4 NOT NULL REFERENCES tasks(id),
    subcategory_id INT4 NOT NULL
);

-- Recreate the original foreign key constraint
ALTER TABLE tasks ADD CONSTRAINT orders_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);

-- Remove the unique constraint from users table
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_user_uid_key;
