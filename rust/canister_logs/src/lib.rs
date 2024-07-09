use ic_cdk::{
    api::management_canister::main::raw_rand as ic00_raw_rand, init, post_upgrade, query, update,
};
use std::time::Duration;

const TIMER_INTERVAL_SEC: u64 = 5;

fn setup_timer() {
    ic_cdk_timers::set_timer_interval(Duration::from_secs(TIMER_INTERVAL_SEC), || {
        ic_cdk::print("right before timer trap");
        ic_cdk::trap("timer trap");
    });
}

#[init]
fn init() {
    setup_timer();
}

#[post_upgrade]
fn post_upgrade() {
    setup_timer();
}

#[update]
fn print(text: String) {
    ic_cdk::print(text);
}

#[query]
fn print_query(text: String) {
    ic_cdk::print(text);
}

#[update]
fn trap(message: String) {
    ic_cdk::print("right before trap");
    ic_cdk::trap(&message);
}

#[query]
fn trap_query(message: String) {
    ic_cdk::print("right before trap_query");
    ic_cdk::trap(&message);
}

#[update]
fn panic(message: String) {
    ic_cdk::print("right before panic");
    panic!("{}", message);
}

#[update]
fn memory_oob() {
    ic_cdk::print("right before memory out of bounds");
    const BUFFER_SIZE: u32 = 10;
    let mut buffer = vec![0u8; BUFFER_SIZE as usize];
    ic_cdk::api::stable::stable_read(BUFFER_SIZE + 1, &mut buffer); // Reading memory outside of buffer should trap.
}

#[update]
fn failed_unwrap() {
    ic_cdk::print("right before failed unwrap");
    String::from_utf8(vec![0xc0, 0xff, 0xee]).unwrap(); // Invalid utf8 should panic.
}

#[update]
async fn raw_rand() -> Vec<u8> {
    ic_cdk::println!("pre ic.raw_rand() call");
    match ic00_raw_rand().await {
        Ok((bytes,)) => {
            ic_cdk::println!("ic.raw_rand() call succeeded");
            bytes
        }
        Err(err) => {
            ic_cdk::println!("ic.raw_rand() call failed: {:?}", err);
            vec![]
        }
    }
}
