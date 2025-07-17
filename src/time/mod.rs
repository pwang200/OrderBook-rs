use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Trait for providing time information to the order book
pub trait TimeProvider: Send + Sync {
    /// Get the current time in milliseconds
    fn current_time(&self) -> u64;
}

/// System time provider that uses the system clock
pub struct SystemTimeProvider;

impl TimeProvider for SystemTimeProvider {
    fn current_time(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }
}

/// Block time provider for blockchain environments
/// Provides deterministic time based on blockchain block timestamps
pub struct BlockTimeProvider {
    time: Arc<AtomicU64>,
}

impl BlockTimeProvider {
    /// Create a new block time provider with initial time
    pub fn new(initial_time: u64) -> Self {
        Self {
            time: Arc::new(AtomicU64::new(initial_time)),
        }
    }

    /// Create a block time provider initialized with current system time
    pub fn new_with_system_time() -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        Self::new(current_time)
    }

    /// Set the current block time
    pub fn set_time(&self, time: u64) {
        let current = self.time.load(Ordering::SeqCst);
        if time < current {
            tracing::warn!("Block time moving backward: {} -> {}", current, time);
        }
        self.time.store(time, Ordering::SeqCst);
    }

    /// Advance the block time by the given amount
    pub fn advance_time(&self, increment: u64) {
        self.time.fetch_add(increment, Ordering::SeqCst);
    }
}

impl TimeProvider for BlockTimeProvider {
    fn current_time(&self) -> u64 {
        self.time.load(Ordering::SeqCst)
    }
}

/// Mock time provider for testing
pub struct MockTimeProvider {
    time: Arc<AtomicU64>,
}

impl MockTimeProvider {
    /// Create a new mock time provider with initial time
    pub fn new(initial_time: u64) -> Self {
        Self {
            time: Arc::new(AtomicU64::new(initial_time)),
        }
    }

    /// Set the mock time
    pub fn set_time(&self, time: u64) {
        self.time.store(time, Ordering::SeqCst);
    }

    /// Advance the mock time by the given amount
    pub fn advance_time(&self, increment: u64) {
        self.time.fetch_add(increment, Ordering::SeqCst);
    }
}

impl TimeProvider for MockTimeProvider {
    fn current_time(&self) -> u64 {
        self.time.load(Ordering::SeqCst)
    }
}

impl Clone for MockTimeProvider {
    fn clone(&self) -> Self {
        Self {
            time: Arc::clone(&self.time),
        }
    }
}

impl Clone for BlockTimeProvider {
    fn clone(&self) -> Self {
        Self {
            time: Arc::clone(&self.time),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_system_time_provider() {
        let provider = SystemTimeProvider;
        let time1 = provider.current_time();
        thread::sleep(Duration::from_millis(5));
        let time2 = provider.current_time();
        assert!(time2 > time1);
    }

    #[test]
    fn test_block_time_provider() {
        let provider = BlockTimeProvider::new(1000);
        assert_eq!(provider.current_time(), 1000);

        provider.set_time(2000);
        assert_eq!(provider.current_time(), 2000);

        provider.advance_time(500);
        assert_eq!(provider.current_time(), 2500);
    }

    #[test]
    fn test_mock_time_provider() {
        let provider = MockTimeProvider::new(5000);
        assert_eq!(provider.current_time(), 5000);

        provider.set_time(10000);
        assert_eq!(provider.current_time(), 10000);

        provider.advance_time(1000);
        assert_eq!(provider.current_time(), 11000);
    }

    #[test]
    fn test_block_time_provider_clone() {
        let provider1 = BlockTimeProvider::new(1000);
        let provider2 = provider1.clone();

        provider1.set_time(2000);
        assert_eq!(provider2.current_time(), 2000);
    }

    #[test]
    fn test_mock_time_provider_clone() {
        let provider1 = MockTimeProvider::new(1000);
        let provider2 = provider1.clone();

        provider1.set_time(2000);
        assert_eq!(provider2.current_time(), 2000);
    }
}
