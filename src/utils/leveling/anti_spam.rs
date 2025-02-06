use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread::sleep;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

pub struct AntiSpam {
    user_messages: HashMap<String, Vec<Instant>>,
    message_limit: usize,
    time_frame: Duration,
}

impl AntiSpam {
    pub fn new(message_limit: usize, time_frame: Duration) -> Self {
        Self {
            user_messages: HashMap::new(),
            message_limit,
            time_frame,
        }
    }

    /// Determines when the user is spamming.
    /// 
    /// It calculates it based on `message_limit` and `time_frame`
    /// given on `AntiSpam`. 
    pub fn is_spamming(&mut self, user_id: &str) -> bool {
        let now = Instant::now();
        let messages = self.user_messages.entry(user_id.to_string()).or_insert(Vec::new());

        messages.retain(|&timestamp| now.duration_since(timestamp) < self.time_frame);

        if messages.len() >= self.message_limit {
            true
        } else {
            messages.push(now);
            false
        }
    }
}

pub static ANTI_SPAM: Lazy<Arc<Mutex<AntiSpam>>> = Lazy::new(|| {
    Arc::new(Mutex::new(AntiSpam::new(2, Duration::from_secs(30))))
});

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anti_spam() {
        let user_id = "user1";

        for _ in 0..5 {
            assert_eq!(ANTI_SPAM.lock().await.is_spamming(user_id), false);
        }

        assert_eq!(ANTI_SPAM.lock().await.is_spamming(user_id), true);

        sleep(Duration::from_secs(10));

        assert_eq!(ANTI_SPAM.lock().await.is_spamming(user_id), false);
    }
}