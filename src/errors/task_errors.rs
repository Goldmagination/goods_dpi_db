use crate::errors::firebase_errors::FirebaseServiceError;
use chrono::ParseError;
use diesel::result::Error as DieselError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Database error: {0}")]
    DieselError(#[from] DieselError),

    #[error("Firebase upload error: {0}")]
    FirebaseUploadError(#[from] FirebaseServiceError),

    #[error("Invalid date format: {0}")]
    InvalidDate(#[from] ParseError),

    #[error("Invalid category ID: {0}")]
    InvalidCategoryId(#[from] ParseIntError),

    #[error("Blocking error: {0}")]
    BlockingError(String),

    #[error("Database pool error: {0}")]
    DatabasePoolError(String),
}
