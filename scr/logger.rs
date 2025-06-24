use chrono::Local;
use std::fs::{OpenOptions};
use std::io::Write;

pub fn log_thought_action_observation(thought: &str, action: &str, observation: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let entry = format!(
        "[{}] Thought: {}
[{}] Action: {}
[{}] Observation: {}

",
        timestamp, thought, timestamp, action, timestamp, observation
    );

    let log_path = "thought_logs.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("Unable to open log file");
    file.write_all(entry.as_bytes())
        .expect("Unable to write to log file");
}