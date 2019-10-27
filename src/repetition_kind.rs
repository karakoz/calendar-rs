use chrono::Duration;

#[derive(Clone)]
pub enum RepetitionKind {
    Duration(Duration),
    Years(u32),
    Months(u32),
}



