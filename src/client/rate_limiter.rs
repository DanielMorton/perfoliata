use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests_per_second: f64,
    last_request: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    pub fn new(requests_per_second: f64) -> Self {
        Self {
            requests_per_second,
            last_request: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub async fn wait(&self) {
        let mut last_request = self.last_request.lock().await;
        let current_time = Instant::now();
        let time_since_last = current_time.duration_since(*last_request);
        let min_interval = Duration::from_secs_f64(1.0 / self.requests_per_second);

        if time_since_last < min_interval {
            let sleep_duration = min_interval - time_since_last;
            drop(last_request); // Release lock before sleeping
            sleep(sleep_duration).await;
            *self.last_request.lock().await = Instant::now();
        } else {
            *last_request = current_time;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_rate_limiter() {
        let rate_limiter = RateLimiter::new(2.0); // 2 requests per second
        let start = Instant::now();

        rate_limiter.wait().await;
        rate_limiter.wait().await;
        rate_limiter.wait().await;

        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(1000)); // Should take at least 1 second for 3 requests at 2 RPS
    }
}
