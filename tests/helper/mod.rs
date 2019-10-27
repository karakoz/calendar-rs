
use chrono::{NaiveDate, NaiveDateTime};

pub fn dt_from_ymd_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> NaiveDateTime {
    NaiveDate::from_ymd(year, month, day).and_hms(hour, min, sec)
}