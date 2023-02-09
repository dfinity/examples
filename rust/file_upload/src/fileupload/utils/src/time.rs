use types::{TimestampMillis, TimestampNanos};

pub const SECOND_IN_MS: u64 = 1000;
pub const MINUTE_IN_MS: u64 = SECOND_IN_MS * 60;
pub const HOUR_IN_MS: u64 = MINUTE_IN_MS * 60;
pub const DAY_IN_MS: u64 = HOUR_IN_MS * 24;
pub const WEEK_IN_MS: u64 = DAY_IN_MS * 7;

const NANOS_PER_MILLISECOND: u64 = 1_000_000;

pub fn now_millis() -> TimestampMillis {
    ic_cdk::api::time() as u64 / NANOS_PER_MILLISECOND
}

pub fn now_nanos() -> TimestampNanos {
    ic_cdk::api::time() as u64
}
