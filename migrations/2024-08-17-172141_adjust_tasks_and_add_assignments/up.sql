-- Add unique constraint to users table
ALTER TABLE users ADD CONSTRAINT users_user_uid_key UNIQUE (user_uid);

-- Drop the foreign key constraint
ALTER TABLE tasks DROP CONSTRAINT IF EXISTS orders_user_id_fkey;

-- Rename `tasks` table to `task`
ALTER TABLE tasks RENAME TO task;

-- Change user_id type to VARCHAR(255)
ALTER TABLE task ALTER COLUMN user_id TYPE VARCHAR(255) USING user_id::varchar(255);

-- Rename user_id to user_uid
ALTER TABLE task RENAME COLUMN user_id TO user_uid;


-- Add new columns to the `task` table
ALTER TABLE task ADD COLUMN title VARCHAR NOT NULL DEFAULT '';
ALTER TABLE task ADD COLUMN min_price DOUBLE PRECISION;
ALTER TABLE task ADD COLUMN max_price DOUBLE PRECISION;
ALTER TABLE task ADD COLUMN is_flexible_timing BOOLEAN NOT NULL DEFAULT true;
ALTER TABLE task ADD COLUMN scheduled_date DATE;
ALTER TABLE task ADD COLUMN scheduled_time TIME;
ALTER TABLE task ADD COLUMN category_id INT4 NOT NULL DEFAULT 0;

-- Change creation_time to TIMESTAMPTZ
ALTER TABLE task ALTER COLUMN creation_time TYPE TIMESTAMPTZ USING creation_time AT TIME ZONE 'UTC';
ALTER TABLE task
ALTER COLUMN creation_time SET DEFAULT NOW();

-- Drop the `tasks_subcategories` table
DROP TABLE IF EXISTS tasks_subcategories;

-- Create the `task_assignments` table
CREATE TABLE task_assignments (
    id SERIAL PRIMARY KEY,
    task_id INT4 NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    image_url VARCHAR NOT NULL
);

-- Recreate the foreign key constraint
ALTER TABLE task ADD CONSTRAINT task_user_uid_fkey FOREIGN KEY (user_uid) REFERENCES users(user_uid);
