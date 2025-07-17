# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building and Testing
- `make build` - Build the project in debug mode
- `make release` - Build the project in release mode with optimizations
- `make test` - Run all tests (sets LOGLEVEL=WARN to reduce noise)
- `make check` - Run comprehensive checks: tests, formatting, and linting

### Code Quality
- `make fmt` - Format code using rustfmt
- `make fmt-check` - Check formatting without applying changes
- `make lint` - Run clippy with warnings treated as errors
- `make lint-fix` - Auto-fix lint issues where possible
- `make fix` - Apply Rust compiler suggestions

### Benchmarking
- `make bench` - Run benchmarks using Criterion
- `make bench-show` - Open benchmark HTML report
- `make bench-compare` - Compare benchmark runs with verbose output

### Coverage
- `make coverage` - Generate XML coverage report using cargo-tarpaulin
- `make coverage-html` - Generate HTML coverage report
- `make open-coverage` - Open HTML coverage report in browser

### Documentation
- `make doc-open` - Build and open Rust documentation
- `make readme` - Regenerate README.md from source code documentation

### Single Test Execution
To run a single test, use: `cargo test test_name`
To run tests for a specific module: `cargo test module_name`

## Architecture Overview

This is a high-performance, lock-free order book implementation for financial trading systems. The architecture is designed around concurrent data structures and atomic operations to achieve maximum throughput.

### Core Components

#### OrderBook (`src/orderbook/book.rs`)
- Central data structure managing bid/ask price levels
- Uses `DashMap` for lock-free concurrent access to price levels
- Maintains order location mapping for O(1) order lookups
- Tracks last trade price and market close state atomically

#### Price Level Management
- Each price level is managed by the external `pricelevel` crate
- Price levels are stored as `Arc<PriceLevel>` for shared ownership
- Supports complex order types: Standard, Iceberg, PostOnly, FillOrKill, etc.

#### Order Operations (`src/orderbook/operations.rs`)
- Implements order lifecycle: add, modify, cancel
- Handles different order types with specialized logic
- Provides atomic order matching across price levels

#### Concurrent Architecture
- **Bids/Asks**: `DashMap<u64, Arc<PriceLevel>>` - concurrent maps keyed by price
- **Order Locations**: `DashMap<OrderId, (u64, Side)>` - fast order lookup
- **Atomic State**: Uses `AtomicU64` and `AtomicBool` for lock-free state management

### Key Design Patterns

1. **Lock-Free Operations**: Heavy use of atomic operations and concurrent data structures
2. **Shared Ownership**: `Arc<PriceLevel>` enables multiple threads to access price levels safely
3. **Order Location Tracking**: Separate index maintains order-to-price mappings for O(1) lookups
4. **Time-Based Logic**: Utilities in `src/utils/time.rs` for timestamp management

### Testing Strategy

Tests are organized by module:
- Unit tests in `src/orderbook/tests/` cover individual components
- Integration tests in `tests/unit/` test cross-module functionality
- Benchmarks in `benches/` include HFT simulation and contention tests

### Performance Characteristics

Based on benchmarks (Apple M4 Max):
- ~1M operations/second in HFT simulation
- Optimal performance with mostly reads (95%) or mostly writes (0%)
- Counter-intuitive: higher performance with increased contention due to cache effects

### Key Dependencies

- `dashmap` - Lock-free concurrent HashMap
- `pricelevel` - External crate managing individual price levels
- `uuid` - Order ID generation
- `tracing` - Structured logging
- `serde` - Serialization support