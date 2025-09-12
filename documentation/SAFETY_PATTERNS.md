# Safety Patterns Quick Reference

## Overview

This document provides quick reference patterns for preventing infinite loops and ensuring safe operations in the Echoes RPG codebase.

## Core Safety Patterns

### 1. Bounded Loop Pattern

**Use this for:** Any loop that might not terminate naturally

```rust
// ✅ SAFE: Bounded attempts
let max_attempts = 20;
for attempt in 0..max_attempts {
    if try_operation() {
        break; // Success
    }
}

// ❌ UNSAFE: Unbounded loop
while !condition {
    // Could loop forever
}
```

### 2. Platform-Specific Limits

**Use this for:** Operations that need different limits on different platforms

```rust
// ✅ SAFE: Platform-specific limits
let max_entities = if cfg!(target_arch = "wasm32") {
    100  // Lower limit for WASM
} else {
    500  // Higher limit for native
};

// Conditional compilation for constants
#[cfg(target_arch = "wasm32")]
const MAP_SIZE: usize = 40;
#[cfg(not(target_arch = "wasm32"))]
const MAP_SIZE: usize = 80;
```

### 3. Timeout Pattern

**Use this for:** Operations that might block indefinitely

```rust
// ✅ SAFE: With timeout
use std::time::{Duration, Instant};

let timeout = Duration::from_millis(100);
let start = Instant::now();

loop {
    if start.elapsed() > timeout {
        break; // Timeout reached
    }
    
    if poll_operation()? {
        // Process operation
        break;
    }
    
    std::thread::sleep(Duration::from_millis(1));
}
```

### 4. Resource Counting Pattern

**Use this for:** Managing limited resources (timers, handles, etc.)

```rust
struct ResourceManager {
    active_resources: u32,
    max_resources: u32,
}

impl ResourceManager {
    fn acquire_resource(&mut self) -> Result<ResourceHandle, Error> {
        if self.active_resources >= self.max_resources {
            return Err(Error::ResourceLimitExceeded);
        }
        
        self.active_resources += 1;
        Ok(ResourceHandle::new(/* ... */))
    }
    
    fn release_resource(&mut self) {
        if self.active_resources > 0 {
            self.active_resources -= 1;
        }
    }
}
```

### 5. Rate Limiting Pattern

**Use this for:** Preventing excessive operations

```rust
struct RateLimiter {
    last_operation: f64,
    min_interval: f64,
    operation_count: u32,
    last_reset: f64,
}

impl RateLimiter {
    fn can_proceed(&mut self) -> bool {
        let now = current_time();
        
        // Reset counter every second
        if now - self.last_reset > 1000.0 {
            self.operation_count = 0;
            self.last_reset = now;
        }
        
        // Check rate limits
        if now - self.last_operation < self.min_interval {
            return false;
        }
        
        if self.operation_count > 100 {
            return false;
        }
        
        self.last_operation = now;
        self.operation_count += 1;
        true
    }
}
```

### 6. Error Recovery Pattern

**Use this for:** Operations that might fail repeatedly

```rust
fn robust_operation() -> Result<(), Error> {
    let mut consecutive_errors = 0;
    const MAX_ERRORS: u32 = 5;
    
    loop {
        match try_operation() {
            Ok(result) => {
                consecutive_errors = 0; // Reset on success
                return Ok(result);
            }
            Err(e) => {
                consecutive_errors += 1;
                
                if consecutive_errors >= MAX_ERRORS {
                    return Err(Error::TooManyErrors);
                }
                
                // Exponential backoff
                let delay = Duration::from_millis(100 * consecutive_errors as u64);
                std::thread::sleep(delay);
            }
        }
    }
}
```

### 7. Graceful Degradation Pattern

**Use this for:** Non-critical operations that can fail safely

```rust
fn place_optional_entities() {
    for entity in entities_to_place {
        match try_place_entity(entity) {
            Ok(_) => {
                // Success - continue
            }
            Err(e) => {
                // Log error but continue
                log_warning!("Failed to place entity: {}", e);
                // Don't fail the entire operation
            }
        }
    }
}
```

### 8. WASM-Safe Logging Pattern

**Use this for:** Cross-platform logging

```rust
macro_rules! safe_log {
    ($msg:expr) => {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&$msg.into());
        
        #[cfg(not(target_arch = "wasm32"))]
        eprintln!("{}", $msg);
    };
}

// Usage
safe_log!("Warning: Operation failed");
safe_log!(format!("Processing item {}", item_id));
```

### 9. Processing Batch Pattern

**Use this for:** Large operations that need to be chunked

```rust
fn process_large_dataset(data: &[Item]) -> Result<(), Error> {
    const BATCH_SIZE: usize = 100;
    const MAX_BATCHES: usize = 50;
    
    let mut processed_batches = 0;
    
    for batch in data.chunks(BATCH_SIZE) {
        if processed_batches >= MAX_BATCHES {
            safe_log!("Batch limit reached, stopping processing");
            break;
        }
        
        process_batch(batch)?;
        processed_batches += 1;
        
        // Yield CPU for WASM
        #[cfg(target_arch = "wasm32")]
        if processed_batches % 5 == 0 {
            // Allow browser to process other events
            return Ok(()); // Resume in next frame
        }
    }
    
    Ok(())
}
```

### 10. Fallback Creation Pattern

**Use this for:** Operations that must succeed with a fallback

```rust
fn create_level_with_fallback() -> Level {
    // Try complex generation first
    if let Ok(level) = try_generate_complex_level() {
        return level;
    }
    
    // Fall back to simple generation
    if let Ok(level) = try_generate_simple_level() {
        return level;
    }
    
    // Last resort: minimal level
    create_minimal_level()
}

fn create_minimal_level() -> Level {
    // This MUST always succeed
    let mut level = Level::new(10, 10);
    level.add_room(Room::new(2, 2, 6, 6));
    level.place_player_at(Position::new(4, 4));
    level
}
```

## Anti-Patterns to Avoid

### ❌ Unbounded Loops
```rust
// DON'T DO THIS
while !found_position {
    try_random_position();
}

// DON'T DO THIS
loop {
    if condition {
        break;
    }
    // No bounds or timeout
}
```

### ❌ Infinite Recursion
```rust
// DON'T DO THIS
fn recursive_function(depth: u32) {
    recursive_function(depth + 1); // No base case
}

// DO THIS INSTEAD
fn safe_recursive_function(depth: u32, max_depth: u32) {
    if depth >= max_depth {
        return; // Base case
    }
    safe_recursive_function(depth + 1, max_depth);
}
```

### ❌ Resource Leaks
```rust
// DON'T DO THIS
fn create_timer() {
    let timer_id = set_timeout(callback, delay);
    // Timer never cleared!
}

// DO THIS INSTEAD
struct TimerGuard {
    timer_id: Option<i32>,
}

impl Drop for TimerGuard {
    fn drop(&mut self) {
        if let Some(id) = self.timer_id.take() {
            clear_timeout(id);
        }
    }
}
```

### ❌ Blocking Operations
```rust
// DON'T DO THIS (especially in WASM)
fn wait_for_input() -> Input {
    loop {
        if let Some(input) = check_input() {
            return input;
        }
        // Blocks browser!
    }
}

// DO THIS INSTEAD
fn poll_for_input(timeout_ms: u64) -> Option<Input> {
    let start = current_time();
    
    loop {
        if let Some(input) = check_input() {
            return Some(input);
        }
        
        if current_time() - start > timeout_ms as f64 {
            return None;
        }
        
        yield_cpu(); // Let other operations run
    }
}
```

## Platform-Specific Considerations

### WASM Specific

```rust
// Use smaller limits
#[cfg(target_arch = "wasm32")]
const PROCESSING_LIMIT: usize = 500;

// Yield CPU regularly
#[cfg(target_arch = "wasm32")]
fn yield_cpu() {
    // In real implementation, this might return early
    // to allow the next animation frame
}

// Use console.log instead of println!
#[cfg(target_arch = "wasm32")]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        web_sys::console::log_1(&format!($($arg)*).into());
    };
}
```

### Native Desktop

```rust
// Can use larger limits
#[cfg(not(target_arch = "wasm32"))]
const PROCESSING_LIMIT: usize = 5000;

// Can use threading
#[cfg(not(target_arch = "wasm32"))]
fn yield_cpu() {
    std::thread::sleep(Duration::from_millis(1));
}

// Use standard logging
#[cfg(not(target_arch = "wasm32"))]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
    };
}
```

## Testing Safety Patterns

### Stress Testing
```rust
#[cfg(test)]
mod stress_tests {
    #[test]
    fn test_bounded_operation_under_stress() {
        for _ in 0..1000 {
            let result = bounded_operation();
            assert!(result.is_ok());
        }
    }
    
    #[test]
    fn test_resource_limits() {
        let mut manager = ResourceManager::new(5);
        
        // Should succeed up to limit
        for _ in 0..5 {
            assert!(manager.acquire_resource().is_ok());
        }
        
        // Should fail at limit
        assert!(manager.acquire_resource().is_err());
    }
}
```

### Edge Case Testing
```rust
#[cfg(test)]
mod edge_case_tests {
    #[test]
    fn test_empty_collections() {
        let empty_data = vec![];
        assert!(process_data(&empty_data).is_ok());
    }
    
    #[test]
    fn test_maximum_values() {
        let max_data = vec![Item::default(); 10000];
        assert!(process_data(&max_data).is_ok());
    }
}
```

## Quick Checklist

Before implementing any loop or potentially blocking operation:

- [ ] Does this loop have explicit bounds?
- [ ] Is there a timeout mechanism?
- [ ] What happens if resources are exhausted?
- [ ] Is error recovery implemented?
- [ ] Are platform differences considered?
- [ ] Is there appropriate logging?
- [ ] Can this operation be interrupted safely?
- [ ] Is there a fallback mechanism?
- [ ] Have edge cases been tested?
- [ ] Will this work in WASM's single-threaded environment?

---

**Remember:** It's better to fail gracefully than to hang indefinitely!