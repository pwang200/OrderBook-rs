use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Global block time state
static CURRENT_BLOCK_TIME: AtomicU64 = AtomicU64::new(0);

/// Returns the current block time in milliseconds since UNIX epoch
pub fn current_block_time() -> u64 {
    CURRENT_BLOCK_TIME.load(Ordering::SeqCst)
}

/// Sets the current block time with validation
pub fn set_current_block_time(block_time: u64) {
    let current = CURRENT_BLOCK_TIME.load(Ordering::SeqCst);
    if block_time < current {
        tracing::warn!("Block time moving backward: {} -> {}", current, block_time);
    }
    CURRENT_BLOCK_TIME.store(block_time, Ordering::SeqCst);
}

/// Initialize block time with reasonable default
pub fn init_block_time(initial_time: Option<u64>) {
    let time = initial_time.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    });
    set_current_block_time(time);
}

/// Returns the current time in milliseconds since UNIX epoch
/// Now uses block time instead of system time for blockchain determinism
pub fn current_time_millis() -> u64 {
    current_block_time()
}

/// Test utility to set block time for deterministic testing
#[cfg(test)]
pub fn set_test_block_time(block_time: u64) {
    set_current_block_time(block_time);
}

/// Test utility to advance block time by increment
#[cfg(test)]
pub fn advance_test_block_time(increment: u64) {
    let current = current_block_time();
    set_current_block_time(current + increment);
}
