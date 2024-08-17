use crate::schema::schema::task;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub is_flexible_timing: bool,
    pub scheduled_date: Option<chrono::NaiveDate>,
    pub scheduled_time: Option<chrono::NaiveTime>,
    pub category_id: Option<i32>,
    pub creation_time: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = task]
pub struct NewTask {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub is_flexible_timing: bool,
    pub scheduled_date: Option<chrono::NaiveDate>,
    pub scheduled_time: Option<chrono::NaiveTime>,
    pub category_id: Option<i32>,
}
