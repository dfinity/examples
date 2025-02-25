use candid::types::number::Nat;
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0_u32));
}

/// Get the value of the counter.
#[ic_cdk_macros::query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

/// Set the value of the counter.
#[ic_cdk_macros::update]
fn set(n: Nat) {
    // COUNTER.replace(n);  // requires #![feature(local_key_cell_methods)]
    COUNTER.with(|count| *count.borrow_mut() = n);
}

#[ic_cdk_macros::update]
fn get_and_set(n: Nat) -> Nat {
    COUNTER.with(|counter| {
        let old = counter.borrow().clone();
        *counter.borrow_mut() = n;
        old
    })
}

/// Increment the value of the counter.
#[ic_cdk_macros::update]
fn increment() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1_u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let expected = Nat::from(42_u32);
        set(expected.clone());
        assert_eq!(get(), expected);
    }

    #[test]
    fn test_init() {
        assert_eq!(get(), Nat::from(0_u32));
    }

    #[test]
    fn test_inc() {
        for i in 1..10_u32 {
            inc();
            assert_eq!(get(), Nat::from(i));
        }
    }

    #[test]
    fn test_get_and_set() {
        let old = get_and_set(Nat::from(1 as u32));
        let new = get();
        assert_eq!(old, Nat::from(0 as u32));
        assert_eq!(new, Nat::from(1 as u32));
    }
}
