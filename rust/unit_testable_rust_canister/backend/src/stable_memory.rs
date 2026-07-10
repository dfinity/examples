use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableCell,
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
    COUNTER.with_borrow(|counter| f(counter.get()))
}

/// Mutates the counter value  
/// Following NNS governance pattern: with_counter_mut(|counter| ...)
pub fn with_counter_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut u64) -> R,
{
    COUNTER.with(|cell| {
        // Apply the mutation function to a local copy
        let mut local_value = *cell.borrow().get();
        let result = f(&mut local_value);

        // Write back the modified value
        cell.borrow_mut()
            .set(local_value)
            .expect("failed to set counter value");

        result
    })
}
