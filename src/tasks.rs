use chrono::{ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let timestamp: DateTime<Utc> = Utc::now();
        Task { text, timestamp }
    }
}
