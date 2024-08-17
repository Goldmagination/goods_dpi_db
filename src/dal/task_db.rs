use crate::models::dtos::task_dto::TaskDto;
use crate::models::task_aggregate::task::{NewTask, Task};
use crate::schema::schema::{task, task_assignments};
use crate::services::firebase_service::upload_image_to_firebase;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Invalid user UID")]
    InvalidUserUid,
    #[error("Database error: {0}")]
    DieselError(#[from] DieselError),
    #[error("Firebase upload error: {0}")]
    FirebaseUploadError(#[from] Box<dyn std::error::Error>),
}

pub async fn place_task(
    conn: &mut PgConnection,
    user_uid: &str,
    task_dto: TaskDto,
) -> Result<Task, TaskError> {
    let user_id: i32 = user_uid.parse().map_err(|_| TaskError::InvalidUserUid)?;

    let new_task = NewTask {
        user_id,
        title: task_dto.title,
        description: task_dto.description,
        min_price: task_dto.min_price,
        max_price: task_dto.max_price,
        is_flexible_timing: task_dto.is_flexible_timing,
        scheduled_date: task_dto.scheduled_date,
        scheduled_time: task_dto.scheduled_time,
        category_id: task_dto.category_id,
    };

    let inserted_task: Task = diesel::insert_into(task::table)
        .values(&new_task)
        .get_result(conn)?;

    if let Some(image_base64_strings) = task_dto.image_base64_strings {
        for (i, image_string) in image_base64_strings.iter().enumerate() {
            let image_bytes = base64::decode(image_string)
                .map_err(|e| TaskError::FirebaseUploadError(Box::new(e)))?;
            let file_name = format!("task_{}_{}.png", inserted_task.id, i);
            let image_url = upload_image_to_firebase(image_bytes, file_name).await?;

            let new_task_assignment = NewTaskAssignment {
                task_id: inserted_task.id,
                image_url,
            };

            diesel::insert_into(task_assignments::table)
                .values(&new_task_assignment)
                .execute(conn)?;
        }
    }

    Ok(inserted_task)
}
