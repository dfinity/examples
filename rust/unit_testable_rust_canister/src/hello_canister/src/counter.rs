use crate::stable_memory::{with_counter, with_counter_mut};

// This is a toy example to show how we can put stable memory behind an interface to make tests
// easier in places where we want to test business logic.
pub trait Counter {
    fn get_count(&self) -> u64;
    fn increment_count(&self) -> u64;
    fn decrement_count(&self) -> u64;
}

pub struct StableMemoryCounter;

impl Counter for StableMemoryCounter {
    fn get_count(&self) -> u64 {
        with_counter(|counter| *counter)
    }

    fn increment_count(&self) -> u64 {
        with_counter_mut(|counter| {
            *counter += 1;
            *counter
        })
    }

    fn decrement_count(&self) -> u64 {
        with_counter_mut(|counter| {
            *counter = counter.saturating_sub(1);
            *counter
        })
    }
}

#[cfg(test)]
pub mod test_util {
    use crate::counter::Counter;
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    pub struct TestCounter {
        count: Arc<Mutex<u64>>,
    }
    impl TestCounter {
        pub fn new() -> TestCounter {
            Default::default()
        }
    }
    impl Counter for TestCounter {
        fn get_count(&self) -> u64 {
            *self.count.lock().unwrap()
        }

        fn increment_count(&self) -> u64 {
            let mut guard = self.count.lock().unwrap();
            *guard = guard.saturating_add(1);
            *guard
        }

        fn decrement_count(&self) -> u64 {
            let mut guard = self.count.lock().unwrap();
            *guard = guard.saturating_sub(1);
            *guard
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable_counter() {
        let counter = StableMemoryCounter;
        assert_eq!(counter.get_count(), 0);
        counter.increment_count();
        assert_eq!(counter.get_count(), 1);
        counter.increment_count();
        assert_eq!(counter.get_count(), 2);

        counter.decrement_count();
        assert_eq!(counter.get_count(), 1);
        counter.decrement_count();
        assert_eq!(counter.get_count(), 0);
        counter.decrement_count();
        assert_eq!(counter.get_count(), 0);
    }
}
