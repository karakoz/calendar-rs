use chrono::NaiveDateTime;

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct TimeInterval {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime
}