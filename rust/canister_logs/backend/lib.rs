use ic_cdk::{init, post_upgrade, query, update};
use ic_cdk_management_canister::raw_rand as ic00_raw_rand;
use std::time::Duration;

const TIMER_INTERVAL_SEC: u64 = 5;

fn setup_timer() {
    ic_cdk_timers::set_timer_interval(Duration::from_secs(TIMER_INTERVAL_SEC), || async {
        ic_cdk::println!("right before timer trap");
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
    ic_cdk::println!("{}", text);
}

#[query]
fn print_query(text: String) {
    ic_cdk::println!("{}", text);
}

#[update]
fn trap(message: String) {
    ic_cdk::println!("right before trap");
    ic_cdk::trap(&message);
}

#[query]
fn trap_query(message: String) {
    ic_cdk::println!("right before trap_query");
    ic_cdk::trap(&message);
}

#[update]
fn panic(message: String) {
    ic_cdk::println!("right before panic");
    panic!("{}", message);
}

#[update]
fn memory_oob() {
    ic_cdk::println!("right before memory out of bounds");
    const BUFFER_SIZE: u32 = 10;
    let mut buffer = vec![0u8; BUFFER_SIZE as usize];
    ic_cdk::stable::stable_read((BUFFER_SIZE + 1) as u64, &mut buffer); // Reading memory outside of buffer should trap.
}

#[update]
fn failed_unwrap() {
    ic_cdk::println!("right before failed unwrap");
    String::from_utf8(vec![0xc0, 0xff, 0xee]).unwrap(); // Invalid utf8 should panic.
}

#[update]
async fn raw_rand() -> Vec<u8> {
    ic_cdk::println!("pre ic.raw_rand() call");
    match ic00_raw_rand().await {
        Ok(bytes) => {
            ic_cdk::println!("ic.raw_rand() call succeeded");
            bytes
        }
        Err(err) => {
            ic_cdk::println!("ic.raw_rand() call failed: {:?}", err);
            vec![]
        }
    }
}

ic_cdk::export_candid!();
