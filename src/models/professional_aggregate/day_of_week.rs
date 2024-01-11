#[derive(Debug, PartialEq, Eq)]
pub enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Holiday = 7, // Represents a holiday or day off
}

impl DayOfWeek {
    pub fn from_i32(value: i32) -> Option<DayOfWeek> {
        match value {
            0 => Some(DayOfWeek::Sunday),
            1 => Some(DayOfWeek::Monday),
            2 => Some(DayOfWeek::Tuesday),
            3 => Some(DayOfWeek::Wednesday),
            4 => Some(DayOfWeek::Thursday),
            5 => Some(DayOfWeek::Friday),
            6 => Some(DayOfWeek::Saturday),
            7 => Some(DayOfWeek::Holiday),
            _ => None, // Invalid value
        }
    }

    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}
