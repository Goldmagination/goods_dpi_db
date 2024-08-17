use crate::models::dtos::task_dto::TaskDto;
use crate::models::task_aggregate::task::{NewTask, Task};
use crate::schema::schema::{task, task_assignments};
use crate::services::firebase_service::upload_image_to_firebase;
use base64::{engine::general_purpose, Engine};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::error::Error;
use uuid::Uuid;

pub async fn place_task(
    conn: &mut PgConnection,
    user_uid: String,
    task_dto: TaskDto,
) -> Result<Task, DieselError> {
    let new_task = create_new_task(user_uid, &task_dto)?;
    let inserted_task: Task = diesel::insert_into(task::table)
        .values(&new_task)
        .get_result(conn)?;

    if let Some(image_strings) = task_dto.image_base64_strings {
        process_images(conn, &inserted_task, image_strings).await?;
    }

    Ok(inserted_task)
}

fn create_new_task(user_uid: String, task_dto: &TaskDto) -> Result<NewTask, DieselError> {
    Ok(NewTask {
        user_id: user_uid
            .parse()
            .map_err(|_| DieselError::RollbackTransaction)?,
        title: task_dto.title.clone(),
        description: task_dto.description.clone(),
        min_price: task_dto.min_price,
        max_price: task_dto.max_price,
        is_flexible_timing: task_dto.is_flexible_timing,
        scheduled_date: task_dto.scheduled_date,
        scheduled_time: task_dto.scheduled_time,
        category_id: task_dto.category_id,
    })
}

async fn process_images(
    conn: &mut PgConnection,
    task: &Task,
    image_strings: Vec<String>,
) -> Result<(), DieselError> {
    for image_base64 in image_strings {
        let image_bytes = decode_image_base64(image_base64)?;
        let file_name = generate_unique_filename();
        let image_url = upload_image_to_firebase(image_bytes, file_name)
            .await
            .map_err(|e| {
                DieselError::QueryBuilderError(Box::new(e) as Box<dyn Error + Send + Sync>)
            })?;

        let new_task_assignment = NewTaskAssignment {
            task_id: task.id,
            image_url,
        };

        diesel::insert_into(task_assignments::table)
            .values(&new_task_assignment)
            .execute(conn)?;
    }
    Ok(())
}

fn generate_unique_filename() -> String {
    let uuid = Uuid::new_v4();
    format!("tasks/{}.jpg", uuid)
}

fn decode_image_base64(base64_string: String) -> Result<Vec<u8>, DieselError> {
    general_purpose::STANDARD
        .decode(base64_string)
        .map_err(|e| DieselError::QueryBuilderError(Box::new(e) as Box<dyn Error + Send + Sync>))
}
