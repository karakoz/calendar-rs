use crate::offset_kind::OffsetKind;
use crate::repetition_kind::RepetitionKind;
use chrono::Duration;
use serde_derive::{Serialize, Deserialize};
use crate::json_duration;

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct RuleItem {
    pub offset: OffsetKind,
    pub repetition: Option<RepetitionKind>,
    #[serde(with = "json_duration")]
    pub length: Duration, // interval length
}