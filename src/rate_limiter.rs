use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct RateLimitInfo {
    pub count: u32,
    pub last_reset: Instant,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<IpAddr, RateLimitInfo>>>,
    pub max_requests: u32,
    pub window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_minutes: u64) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window_duration: Duration::from_secs(window_minutes * 60),
        }
    }

    pub async fn check_rate_limit(&self, ip: IpAddr) -> Result<(), RateLimitError> {
        let mut requests = self.requests.write().await;

        let info = requests.entry(ip).or_insert(RateLimitInfo {
            count: 0,
            last_reset: Instant::now(),
        });

        let now = Instant::now();

        if now.duration_since(info.last_reset) >= self.window_duration {
            info.last_reset = now;
            info.count = 0;
        }

        if info.count >= self.max_requests {
            let wait_time = self
                .window_duration
                .saturating_sub(now.duration_since(info.last_reset));
            Err(RateLimitError::LimitExceeded {
                wait_time,
                retry_after: SystemTime::now() + wait_time,
            })
        } else {
            info.count += 1;
            Ok(())
        }
    }
}

#[derive(Debug)]
pub enum RateLimitError {
    LimitExceeded {
        wait_time: Duration,
        retry_after: SystemTime,
    },
}

impl RateLimitError {
    pub fn retry_after(&self) -> SystemTime {
        match self {
            RateLimitError::LimitExceeded { retry_after, .. } => *retry_after,
        }
    }
}

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RateLimitError::LimitExceeded { wait_time, .. } => {
                write!(f, "Rate limit exceeded. Try again in {:?}", wait_time)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_basic() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let limiter = RateLimiter::new(5, 60);
            let ip: IpAddr = "127.0.0.1".parse().unwrap();

            for _ in 0..5 {
                assert!(limiter.check_rate_limit(ip).await.is_ok());
            }

            assert!(limiter.check_rate_limit(ip).await.is_err());
        });
    }

    #[test]
    fn test_rate_limit_reset() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let limiter = RateLimiter::new(2, 1);
            let ip: IpAddr = "127.0.0.1".parse().unwrap();

            assert!(limiter.check_rate_limit(ip).await.is_ok());
            assert!(limiter.check_rate_limit(ip).await.is_ok());

            assert!(limiter.check_rate_limit(ip).await.is_err());

            tokio::time::sleep(Duration::from_secs(61)).await;

            assert!(limiter.check_rate_limit(ip).await.is_ok());
        });
    }
}
