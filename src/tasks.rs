use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Task {
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let timestamp: DateTime<Utc> = Utc::now();
        Task { text, timestamp }
    }
}
