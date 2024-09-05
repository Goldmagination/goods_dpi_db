use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BookingStatus {
    Proposed = 0,
    Accepted = 1,
    Rejected = 2,
    CounterOffer = 3,
    InProgress = 4,
    Completed = 5,
    Cancelled = 6,
    Warning = 7,
}

impl From<i32> for BookingStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => BookingStatus::Proposed,
            1 => BookingStatus::Accepted,
            2 => BookingStatus::Rejected,
            3 => BookingStatus::CounterOffer,
            4 => BookingStatus::InProgress,
            5 => BookingStatus::Completed,
            6 => BookingStatus::Cancelled,
            7 => BookingStatus::Warning,
            _ => panic!("Invalid status value: {}", value),
        }
    }
}

impl From<BookingStatus> for i32 {
    fn from(status: BookingStatus) -> Self {
        status as i32
    }
}
