use chrono::DateTime;
use chrono::Utc;
use std::time::Duration;

pub fn format_duration(duration: Duration) -> String {
    let mut duration_secs = duration.as_secs();
    let days = duration_secs / (24 * 3600);
    duration_secs %= 24 * 3600;
    let hours = duration_secs / 3600;
    duration_secs %= 3600;
    let minutes = duration_secs / 60;
    let seconds = duration_secs % 60;

    let mut result = String::new();
    if days > 0 {
        result.push_str(&format!("{:02}:", days));
    }
    if hours > 0 || days > 0 {
        result.push_str(&format!("{:02}:", hours));
    }
    result.push_str(&format!("{:02}:{:02}", minutes, seconds));

    result
}

pub fn duration_to_rfc3339(duration: Duration) -> String {
    let now: DateTime<Utc> = Utc::now();
    let datetime = now + duration;
    datetime.to_rfc3339()
}

pub fn duration_to_datetime(duration: std::time::Duration) -> chrono::DateTime<Utc> {
    let now: DateTime<Utc> = Utc::now();
    now + duration
}
