use super::address_db;
use crate::errors::task_errors::TaskError;
use crate::models::dtos::task_dto::TaskDto;
use crate::models::task_aggregate::task::{NewTask, Task};
use crate::models::task_aggregate::task_assignment::NewTaskAssignments;
use crate::schema::schema::{task, task_assignments};
use actix_web::web;
use chrono::{NaiveDate, NaiveTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub async fn place_task(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user_uid: String,
    task_dto: TaskDto,
) -> Result<Task, TaskError> {
    let task_creation_result = web::block({
        let db_pool = db_pool.clone();
        move || -> Result<Task, TaskError> {
            let mut conn = db_pool
                .get()
                .map_err(|e| TaskError::DatabasePoolError(e.to_string()))?;
            conn.transaction::<_, TaskError, _>(|conn| {
                let address_id = if let Some(ref address) = task_dto.address {
                    match address_db::find_address(conn, address)? {
                        Some(id) => Some(id),
                        None => Some(address_db::insert_address(conn, address)?),
                    }
                } else {
                    None
                };
                let scheduled_date = parse_date(&task_dto.scheduled_date)?;
                let scheduled_time = parse_time(&task_dto.scheduled_time)?;
                let new_task = NewTask {
                    user_uid: user_uid.clone(),
                    creation_time: Utc::now().naive_utc(),
                    description: task_dto.description.clone(),
                    address_id,
                    title: task_dto.title.clone(),
                    min_price: task_dto.min_price,
                    max_price: task_dto.max_price,
                    is_flexible_timing: task_dto.is_flexible_timing,
                    scheduled_date,
                    scheduled_time,
                    category_id: task_dto.category_id,
                };
                // Insert the task into the database
                diesel::insert_into(task::table)
                    .values(&new_task)
                    .get_result(conn)
                    .map_err(TaskError::DieselError)
            })
        }
    })
    .await
    .map_err(|e| TaskError::BlockingError(format!("Blocking error: {}", e)))?;

    let task = match task_creation_result {
        Ok(task) => task,
        Err(e) => return Err(e),
    };

    // Step 2: Perform asynchronous image uploads
    if let Some(image_urls) = task_dto.image_strings {
        let image_urls = image_urls.clone(); // Clone to get owned data

        for image_url in image_urls {
            let db_pool = db_pool.clone();
            let task_id = task.id;
            let image_url_clone = image_url.clone();

            web::block(move || insert_task_assignment(&db_pool, task_id, image_url_clone))
                .await
                .map_err(|e| TaskError::BlockingError(format!("Blocking error: {}", e)))?
                .map_err(|e| TaskError::BlockingError(format!("Task image save error: {}", e)))?;
        }
    }

    Ok(task)
}

fn insert_task_assignment(
    db_pool: &Pool<ConnectionManager<PgConnection>>,
    task_id: i32,
    image_url: String,
) -> Result<(), TaskError> {
    let mut conn = db_pool
        .get()
        .map_err(|e| TaskError::DatabasePoolError(e.to_string()))?;

    let new_task_assignment = NewTaskAssignments { task_id, image_url };

    diesel::insert_into(task_assignments::table)
        .values(&new_task_assignment)
        .execute(&mut conn)
        .map_err(TaskError::DieselError)?;

    Ok(())
}

fn parse_date(date_str: &Option<String>) -> Result<Option<NaiveDate>, TaskError> {
    if let Some(date_str) = date_str.as_deref() {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map(Some)
            .map_err(TaskError::from)
    } else {
        Ok(None)
    }
}

fn parse_time(time_str: &Option<String>) -> Result<Option<NaiveTime>, TaskError> {
    if let Some(time_str) = time_str.as_deref() {
        NaiveTime::parse_from_str(time_str, "%H:%M")
            .map(Some)
            .map_err(TaskError::from)
    } else {
        Ok(None)
    }
}

// pub fn get_tasks_by_user(
//     conn: &mut PgConnection,
//     user_uid: &str,
// ) -> Result<Vec<TaskDto>, TaskError> {
//     let tasks: Vec<(Task, Option<TaskAssignments>, Option<Address>)> = task::table
//         .filter(task::user_uid.eq(user_uid))
//         .left_join(task_assignments::table.on(task::id.eq(task_assignments::task_id)))
//         .left_join(addresses::table.on(task::address_id.eq(addresses::id.nullable())))
//         .load(conn)?;

//     // Map the results to TaskDto
//     let task_dtos: Vec<TaskDto> = tasks
//         .into_iter()
//         .map(|(task, task_assignment, address)| {
//             let image_strings = task_assignment.map(|assignment| vec![assignment.image_url]);

//             TaskDto {
//                 title: task.title.clone(),
//                 description: task.description.clone(),
//                 image_strings,
//                 category_id: task.category_id,
//                 is_flexible_timing: task.is_flexible_timing,
//                 scheduled_date: task.scheduled_date.map(|d| d.to_string()),
//                 scheduled_time: task.scheduled_time.map(|t| t.to_string()),
//                 min_price: task.min_price,
//                 max_price: task.max_price,
//                 address: address.map(|addr| AddressDTO::address_to_dto(&addr)),
//             }
//         })
//         .collect();

//     Ok(task_dtos)
// }
