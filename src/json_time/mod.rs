use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};
use chrono::{DateTime, Utc, NaiveDateTime};

fn time_to_json(t: NaiveDateTime) -> String {
    DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
}

pub fn serialize<S: Serializer>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
    time_to_json(time.clone()).serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
    let time: String = Deserialize::deserialize(deserializer)?;
    Ok(DateTime::parse_from_rfc3339(&time).map_err(D::Error::custom)?.naive_utc())
}