use chrono::Duration;
use serde_derive::{Serialize, Deserialize};
use crate::json_duration;

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum RepetitionKind {
    #[serde(with = "json_duration")]
    Duration(Duration),
    Years(u32),
    Months(u32),
}



