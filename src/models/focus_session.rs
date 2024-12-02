use chrono::{DateTime, Local};

pub struct FocusSession {
    pub id: i64,

    pub start: DateTime<Local>,
    pub focus_seconds: u64,
    pub break_seconds: u64,
}
