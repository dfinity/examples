use ic_cdk::update;

#[update]
fn print(text: String) {
    ic_cdk::print(text);
}

#[update]
fn trap(message: String) {
    ic_cdk::print("right before trap");
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
