use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory}, 
    DefaultMemoryImpl, StableCell
};
use std::cell::RefCell;

// =============================================================================
// STABLE MEMORY SETUP (following NNS governance pattern)
// =============================================================================

type Memory = VirtualMemory<DefaultMemoryImpl>;

const COUNTER_MEMORY_ID: MemoryId = MemoryId::new(1);  

thread_local! {
    // Memory manager for stable storage - like NNS governance
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = 
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
        
    // StableCell for counter - persists across upgrades
    static COUNTER: RefCell<StableCell<u64, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(COUNTER_MEMORY_ID)),
            0
        ).expect("failed to init COUNTER cell")
    );
}

// =============================================================================
// STABLE MEMORY ACCESSORS (following NNS governance with_xyz pattern)
// =============================================================================

/// Gets the current counter value
/// Following NNS governance pattern: with_counter(|counter| ...)
pub fn with_counter<F, R>(f: F) -> R
where
    F: FnOnce(&u64) -> R,
{
    COUNTER.with(|cell| {
        let cell_borrow = cell.borrow();
        let counter_value = cell_borrow.get();
        f(counter_value)
    })
}

/// Mutates the counter value  
/// Following NNS governance pattern: with_counter_mut(|counter| ...)
pub fn with_counter_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut u64) -> R,
{
    COUNTER.with(|cell| {
        // Get current value (StableCell.get() returns &T)
        let current_value: u64 = *cell.borrow().get();
        
        // Apply the mutation function to a local copy
        let mut local_value = current_value;
        let result = f(&mut local_value);
        
        // Write back the modified value
        cell.borrow_mut().set(local_value)
            .expect("failed to set counter value");
        
        result
    })
}



// =============================================================================
// HIGH-LEVEL OPERATIONS (business logic for stable memory)
// =============================================================================

/// Increments the counter and returns the new value
pub fn increment_counter() -> u64 {
    with_counter_mut(|counter| {
        *counter += 1;
        *counter
    })
}

/// Gets the current counter value (read-only)
pub fn get_counter() -> u64 {
    with_counter(|counter| *counter)
}

/// Resets all stable memory (useful for testing)
/// Note: This uses the same stable memory across tests in the same thread
/// but different threads (different tests) have isolated stable memory
#[cfg(test)]
pub fn reset_for_test() {
    with_counter_mut(|counter| *counter = 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_operations() {
        reset_for_test();
        
        // Initial state
        assert_eq!(get_counter(), 0);
        
        // Increment operations
        assert_eq!(increment_counter(), 1);
        assert_eq!(increment_counter(), 2);
        assert_eq!(get_counter(), 2);
        
        // Multiple increments
        assert_eq!(increment_counter(), 3);
        assert_eq!(get_counter(), 3);
    }

    #[test]
    fn test_stable_memory_persistence() {
        // This test demonstrates StableCell persistence
        // Note: In actual canister upgrades, this data would persist
        reset_for_test();
        
        increment_counter();
        increment_counter();
        
        assert_eq!(get_counter(), 2);
        
        // In a real canister upgrade scenario, this data would persist
        // across the upgrade boundary thanks to StableCell
    }

    #[test] 
    fn test_with_functions_stable_memory() {
        reset_for_test();
        
        // Test the with_* function pattern with StableCell
        with_counter_mut(|counter| {
            *counter = 42;
        });
        
        let result = with_counter(|counter| *counter * 2);
        assert_eq!(result, 84);
        
        // Verify the value was actually stored
        assert_eq!(get_counter(), 42);
    }

    #[test]
    fn test_memory_manager_isolation() {
        // Each test thread gets its own MemoryManager instance
        // This provides isolation while still using the StableCell pattern
        reset_for_test();
        
        // Set some state
        increment_counter();
        increment_counter();
        increment_counter();
        
        assert_eq!(get_counter(), 3);
        
        // Other tests in different threads won't see this state
        // because each thread gets its own MemoryManager and StableCell instances
    }
}
