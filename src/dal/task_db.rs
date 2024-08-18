use super::address_db;
use crate::errors::task_errors::TaskError;
use crate::models::address::Address;
use crate::models::dtos::address_dto::AddressDTO;
use crate::models::dtos::task_dto::TaskDto;
use crate::models::task_aggregate::task::{NewTask, Task};
use crate::models::task_aggregate::task_assignment::{NewTaskAssignments, TaskAssignments};
use crate::schema::schema::{addresses, task, task_assignments};
use crate::services::firebase_service::upload_image_to_firebase;

use chrono::{NaiveDate, NaiveTime, Utc};
use diesel::prelude::*;

pub async fn place_task(
    conn: &mut PgConnection,
    user_uid: String,
    task_dto: TaskDto,
) -> Result<Task, TaskError> {
    let address_id = if let Some(ref address) = task_dto.address {
        match address_db::find_address(conn, address)? {
            Some(id) => Some(id),
            None => Some(address_db::insert_address(conn, address)?),
        }
    } else {
        None
    };

    let scheduled_date = if let Some(date_str) = task_dto.scheduled_date.as_deref() {
        Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?)
    } else {
        None
    };

    let scheduled_time = if let Some(time_str) = task_dto.scheduled_time.as_deref() {
        Some(NaiveTime::parse_from_str(time_str, "%H:%M")?)
    } else {
        None
    };
    let new_task = NewTask {
        user_uid,
        creation_time: Utc::now().naive_utc(),
        description: task_dto.description,
        address_id,
        title: task_dto.title,
        min_price: task_dto.min_price,
        max_price: task_dto.max_price,
        is_flexible_timing: task_dto.is_flexible_timing,
        scheduled_date,
        scheduled_time,
        category_id: task_dto.category_id,
    };

    let inserted_task: Task = diesel::insert_into(task::table)
        .values(&new_task)
        .get_result(conn)?;

    if let Some(image_base64_strings) = task_dto.image_strings {
        for (i, image_string) in image_base64_strings.iter().enumerate() {
            let file_name = format!("task_{}_{}.png", inserted_task.id, i);
            let image_url = upload_image_to_firebase(image_string, file_name).await?;

            let new_task_assignment = NewTaskAssignments {
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
pub fn get_tasks_by_user(
    conn: &mut PgConnection,
    user_uid: &str,
) -> Result<Vec<TaskDto>, TaskError> {
    let tasks: Vec<(Task, Option<TaskAssignments>, Option<Address>)> = task::table
        .filter(task::user_uid.eq(user_uid))
        .left_join(task_assignments::table.on(task::id.eq(task_assignments::task_id)))
        .left_join(addresses::table.on(task::address_id.eq(addresses::id.nullable())))
        .load(conn)?;

    // Map the results to TaskDto
    let task_dtos: Vec<TaskDto> = tasks
        .into_iter()
        .map(|(task, task_assignment, address)| {
            let image_strings = task_assignment.map(|assignment| vec![assignment.image_url]);

            TaskDto {
                title: task.title.clone(),
                description: task.description.clone(),
                image_strings,
                category_id: task.category_id,
                is_flexible_timing: task.is_flexible_timing,
                scheduled_date: task.scheduled_date.map(|d| d.to_string()),
                scheduled_time: task.scheduled_time.map(|t| t.to_string()),
                min_price: task.min_price,
                max_price: task.max_price,
                address: address.map(|addr| AddressDTO::address_to_dto(&addr)),
            }
        })
        .collect();

    Ok(task_dtos)
}
