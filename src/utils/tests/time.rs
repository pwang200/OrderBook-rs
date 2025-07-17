#[cfg(test)]
mod tests {
    use crate::current_time_millis;
    use crate::utils::{
        advance_test_block_time, current_block_time, init_block_time, set_current_block_time,
        set_test_block_time,
    };
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use tracing::info;

    #[test]
    fn test_current_time_millis_returns_block_time() {
        // Set a specific block time
        set_test_block_time(1234567890);

        // current_time_millis should return the block time
        assert_eq!(current_time_millis(), 1234567890);

        // Sleep shouldn't change the block time
        thread::sleep(Duration::from_millis(5));
        assert_eq!(current_time_millis(), 1234567890);
    }

    #[test]
    fn test_current_time_millis_consistency() {
        // Set a specific block time
        set_test_block_time(9876543210);

        // current_time_millis should consistently return the same block time
        let time1 = current_time_millis();
        let time2 = current_time_millis();

        assert_eq!(time1, time2);
        assert_eq!(time1, 9876543210);
    }

    #[test]
    fn test_current_time_millis_advance_with_block_time() {
        // Set initial block time
        set_test_block_time(5000);
        let time1 = current_time_millis();
        assert_eq!(time1, 5000);

        // Advance block time
        advance_test_block_time(1000);
        let time2 = current_time_millis();
        assert_eq!(time2, 6000);

        // Advance again
        advance_test_block_time(2000);
        let time3 = current_time_millis();
        assert_eq!(time3, 8000);
    }

    #[test]
    fn test_block_time_deterministic_behavior() {
        // Test that block time behaves deterministically
        set_test_block_time(12345);
        let time1 = current_time_millis();

        // System time changes but block time should remain the same
        thread::sleep(Duration::from_millis(10));
        let time2 = current_time_millis();

        assert_eq!(time1, time2);
        assert_eq!(time1, 12345);
    }

    #[test]
    fn test_block_time_manual_updates() {
        // Test manual block time updates
        set_test_block_time(100);
        assert_eq!(current_time_millis(), 100);

        // Advance by 50
        advance_test_block_time(50);
        assert_eq!(current_time_millis(), 150);

        // Set to a new value
        set_test_block_time(999);
        assert_eq!(current_time_millis(), 999);
    }

    #[test]
    fn test_block_time_order_timestamp() {
        // Test that orders use block time for timestamps
        set_test_block_time(7777);

        // The current_time_millis function should return block time
        let timestamp = current_time_millis();
        assert_eq!(timestamp, 7777);

        // This is what will be used in order creation
        thread::sleep(Duration::from_millis(5));
        let timestamp2 = current_time_millis();
        assert_eq!(timestamp2, 7777); // Should be same as system time doesn't affect block time
    }

    #[test]
    fn test_block_time_operations() {
        // Test setting and getting block time
        let test_time = 1000000;
        set_current_block_time(test_time);
        assert_eq!(current_block_time(), test_time);

        // Test that current_time_millis now returns block time
        assert_eq!(current_time_millis(), test_time);

        // Test updating block time
        let new_time = 2000000;
        set_current_block_time(new_time);
        assert_eq!(current_block_time(), new_time);
        assert_eq!(current_time_millis(), new_time);
    }

    #[test]
    fn test_init_block_time() {
        // Test initializing with specific time
        let init_time = 3000000;
        init_block_time(Some(init_time));
        assert_eq!(current_block_time(), init_time);

        // Test initializing with None (should use system time)
        init_block_time(None);
        let block_time = current_block_time();
        assert!(block_time > 0);
    }

    #[test]
    fn test_block_time_test_utilities() {
        // Test set_test_block_time
        let test_time = 5000000;
        set_test_block_time(test_time);
        assert_eq!(current_block_time(), test_time);

        // Test advance_test_block_time
        let increment = 1000;
        advance_test_block_time(increment);
        assert_eq!(current_block_time(), test_time + increment);
    }

    #[test]
    fn test_block_time_backward_warning() {
        // Set initial time
        let initial_time = 4000000;
        set_current_block_time(initial_time);
        assert_eq!(current_block_time(), initial_time);

        // Try to set time backward (should still work but generate warning)
        let backward_time = 3000000;
        set_current_block_time(backward_time);
        assert_eq!(current_block_time(), backward_time);
    }

    #[test]
    fn test_gtd_order_expiration_with_block_time() {
        use crate::OrderBook;
        use crate::time::{MockTimeProvider, TimeProvider};
        use pricelevel::{OrderId, Side, TimeInForce};
        use std::sync::Arc;
        use uuid::Uuid;

        // Create a mock time provider with initial time 1000
        let time_provider = Arc::new(MockTimeProvider::new(1000));
        let order_book = OrderBook::new_with_time_provider("TEST", time_provider.clone());

        // Create a GTD order that expires at block time 2000
        let order_id = OrderId(Uuid::new_v4());
        let result =
            order_book.add_limit_order(order_id, 1000, 10, Side::Buy, TimeInForce::Gtd(2000));
        assert!(result.is_ok(), "GTD order should be added successfully");

        // Verify order is in the book
        let order = order_book.get_order(order_id);
        assert!(order.is_some(), "Order should be in the book");

        // Advance block time to 1500 (before expiration)
        time_provider.advance_time(500);

        // Order should still be in the book (not expired)
        let order = order_book.get_order(order_id);
        assert!(
            order.is_some(),
            "Order should still be in the book before expiration"
        );

        // Advance block time to 2000 (at expiration)
        time_provider.advance_time(500);

        // Order should now be expired when checked
        let order = order_book.get_order(order_id);
        if let Some(order) = order {
            // The order is still in the book, but should be considered expired
            // Note: has_expired is a private method, so we can't test it directly
            // In a real implementation, expired orders would be removed during cleanup
            // For now, we just verify the order is still there but would be expired
            assert!(
                order
                    .time_in_force()
                    .is_expired(time_provider.current_time(), None),
                "Order should be expired at expiration time"
            );
        }
    }
}
