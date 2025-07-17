mod time;

mod tests;

#[allow(unused)]
pub use time::{current_block_time, current_time_millis, init_block_time, set_current_block_time};

#[cfg(test)]
pub use time::{advance_test_block_time, set_test_block_time};
