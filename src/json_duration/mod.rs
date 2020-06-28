use serde::{Serialize, Serializer, Deserialize, Deserializer};
use chrono::{Duration};

fn time_to_json(duration: Duration) -> String {
    duration.num_seconds().to_string()
}

pub fn serialize<S: Serializer>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
    time_to_json(duration.clone()).serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
    let seconds: i64 = Deserialize::deserialize(deserializer)?;
    Ok(Duration::seconds(seconds))
}