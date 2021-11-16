use chrono::{ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufReader, Result, Seek, SeekFrom};

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

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}
