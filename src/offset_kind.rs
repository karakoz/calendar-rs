use chrono::Duration;
use chrono::prelude::*;
use serde_derive::{Serialize, Deserialize};
use crate::json_time;
use crate::json_duration;

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum OffsetKind {
    #[serde(with = "json_time")]
    DateTime(NaiveDateTime),
    #[serde(with = "json_duration")]
    Duration(Duration)
}