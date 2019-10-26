
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub fn dt_from_ymd_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> NaiveDateTime {
    NaiveDateTime::new(NaiveDate::from_ymd(year, month, day), NaiveTime::from_hms(hour, min, sec))
}