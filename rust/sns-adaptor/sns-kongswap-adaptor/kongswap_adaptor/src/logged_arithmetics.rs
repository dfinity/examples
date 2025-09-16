/// Logged versions of saturating arithmetic operations.
/// These functions perform the operations and log an error message if an overflow or underflow occurs.
use crate::log_err;

pub(crate) fn logged_saturating_add(a: u64, b: u64) -> u64 {
    match a.checked_add(b) {
        Some(sum) => sum,
        None => {
            log_err(&format!("saturating_add({} + {}) overflowed", a, b));
            u64::MAX
        }
    }
}

pub(super) fn logged_saturating_sub(a: u64, b: u64) -> u64 {
    match a.checked_sub(b) {
        Some(sub) => sub,
        None => {
            log_err(&format!("saturating_sub({} - {}) underflowed", a, b));
            0
        }
    }
}
