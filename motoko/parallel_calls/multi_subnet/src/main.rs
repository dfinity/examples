use candid::{decode_one, encode_one, Nat, Principal};
use pocket_ic::{PocketIcBuilder, WasmResult};
use std::time::Instant;

const INIT_CYCLES: u128 = 2_000_000_000_000;

fn main() {
    let num_calls: Nat = Nat::from(90 as u64);
    let pic = PocketIcBuilder::new()
        .with_application_subnet()
        .with_application_subnet()
        .build();
    let app_sub_1 = pic.topology().get_app_subnets()[0];
    let app_sub_2 = pic.topology().get_app_subnets()[1];

    let caller_id = pic.create_canister_on_subnet(None, None, app_sub_1);
    pic.add_cycles(caller_id, INIT_CYCLES);
    pic.install_canister(caller_id, caller_wasm(), vec![], None);
    let callee_id = pic.create_canister_on_subnet(None, None, app_sub_2);
    pic.add_cycles(callee_id, INIT_CYCLES);

    pic.install_canister(callee_id, callee_wasm(), vec![], None);
    pic.update_call(
        caller_id,
        Principal::anonymous(),
        "setup_callee",
        encode_one(callee_id).unwrap(),
    )
    .expect("Failed to set the callee canister up");

    let sequential_start = Instant::now();
    let sequential_result = pic
        .update_call(
            caller_id,
            Principal::anonymous(),
            "sequential_calls",
            encode_one(num_calls.clone()).unwrap(),
        )
        .expect("Failed to execute sequential calls");
    let sequential_num_calls: Nat = match sequential_result {
        WasmResult::Reply(reply) => decode_one(&reply).unwrap(),
        WasmResult::Reject(code) => panic!("Unexpected reject code for sequential calls: {}", code),
    };
    let sequential_duration = sequential_start.elapsed();

    let parallel_start = Instant::now();
    let parallel_result = pic
        .update_call(
            caller_id,
            Principal::anonymous(),
            "parallel_calls",
            encode_one(num_calls.clone()).unwrap(),
        )
        .expect("Failed to execute parallel calls");
    let parallel_num_calls: Nat = match parallel_result {
        WasmResult::Reply(reply) => decode_one(&reply).unwrap(),
        WasmResult::Reject(code) => panic!("Unexpected reject code for parallel calls: {}", code),
    };
    let parallel_duration = parallel_start.elapsed();

    println!(
        "Sequential calls: {}/{} successful calls in {:?}",
        sequential_num_calls, num_calls, sequential_duration
    );
    println!(
        "Parallel calls: {}/{} successful calls in {:?}",
        parallel_num_calls, num_calls, parallel_duration
    );
}

fn caller_wasm() -> Vec<u8> {
    let wasm_path = std::env::var_os("CALLER_WASM").expect("Missing caller wasm file");
    std::fs::read(wasm_path).unwrap()
}

fn callee_wasm() -> Vec<u8> {
    let wasm_path = std::env::var_os("CALLEE_WASM").expect("Missing callee wasm file");
    std::fs::read(wasm_path).unwrap()
}
