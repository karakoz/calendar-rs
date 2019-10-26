use chrono::Duration;

#[derive(Clone)]
pub enum RepetitionKind {
    Duration(Duration),
    Years(i32),
    Months(i32),

}



