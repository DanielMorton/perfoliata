use rand::Rng;
use std::time::Duration;

pub fn exponential_backoff(retry_count: u32, base_delay: f64, max_delay: f64) -> Duration {
    let delay = (base_delay * (2.0_f64.powi(retry_count as i32))).min(max_delay);
    let jitter = rand::rng().random_range(0.0..0.1 * delay);
    Duration::from_secs_f64(delay + jitter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff() {
        let delay1 = exponential_backoff(0, 1.0, 60.0);
        let delay2 = exponential_backoff(1, 1.0, 60.0);
        let delay3 = exponential_backoff(2, 1.0, 60.0);

        assert!(delay1.as_secs_f64() >= 1.0 && delay1.as_secs_f64() < 1.2);
        assert!(delay2.as_secs_f64() >= 2.0 && delay2.as_secs_f64() < 2.2);
        assert!(delay3.as_secs_f64() >= 4.0 && delay3.as_secs_f64() < 4.4);
    }
}
