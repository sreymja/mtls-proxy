use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct RateLimiterConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 100,
            burst_size: 200,
        }
    }
}

#[derive(Clone)]
pub struct RateLimiter {
    config: RateLimiterConfig,
    tokens: Arc<RwLock<u32>>,
    last_refill: Arc<RwLock<Instant>>,
}

impl RateLimiter {
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            tokens: Arc::new(RwLock::new(config.burst_size)),
            last_refill: Arc::new(RwLock::new(Instant::now())),
            config,
        }
    }
    
    pub async fn check_async(&self) -> Result<(), RateLimitError> {
        let mut tokens = self.tokens.write().await;
        let mut last_refill = self.last_refill.write().await;
        
        // Refill tokens based on time elapsed
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * self.config.requests_per_second as f64) as u32;
        
        if tokens_to_add > 0 {
            *tokens = (*tokens + tokens_to_add).min(self.config.burst_size);
            *last_refill = now;
        }
        
        // Check if we have tokens available
        if *tokens > 0 {
            *tokens -= 1;
            Ok(())
        } else {
            Err(RateLimitError)
        }
    }
}

#[derive(Debug)]
pub struct RateLimitError;

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rate limit exceeded")
    }
}

impl std::error::Error for RateLimitError {}
