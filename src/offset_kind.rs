use chrono::Duration;
use chrono::prelude::*;

#[derive(Clone)]
pub enum OffsetKind {
    DateTime(NaiveDateTime),
    Duration(Duration)
}