


static mut COUNTER: u64 = 0;

#[ic_cdk_macros::update]
fn increment() -> u64 {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

#[ic_cdk_macros::query]
fn get() -> u64 {
    unsafe { COUNTER }
}

#[ic_cdk_macros::update]
fn reset() -> u64 {
    unsafe {
        COUNTER = 0;
        COUNTER
    }
}

