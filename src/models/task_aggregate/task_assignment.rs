use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct TaskAssignments {
    pub id: i32,
    pub task_id: i32,
    pub image_url: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = task_assignments)] // Updated attribute
pub struct NewTaskAssignments {
    pub task_id: i32,
    pub image_url: String,
}
