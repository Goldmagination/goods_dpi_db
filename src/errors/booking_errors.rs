use chrono::ParseError;
use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BookingError {
    #[error("Database error: {0}")]
    DieselError(#[from] DieselError),

    #[error("Invalid date format: {0}")]
    InvalidDate(#[from] ParseError),

    #[error("Blocking error: {0}")]
    BlockingError(String),

    #[error("Database pool error: {0}")]
    DatabasePoolError(String),
}
