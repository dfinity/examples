use candid::{decode_one, encode_args, encode_one, CandidType, Principal};
use hello_canister::CanisterInfo;
use pocket_ic::{PocketIc, PocketIcBuilder};
use serde::Deserialize;

// Include the WASM binary directly at compile time
const HELLO_CANISTER_WASM: &[u8] = 
    include_bytes!("../../../target/wasm32-unknown-unknown/release/hello_canister.wasm");

/// Test the basic greeting functionality
#[test]
fn test_greet_functionality() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Test greeting with a name
    let response: String = update(&pic, canister_id, "greet", encode_one("Alice").unwrap());
    assert_eq!(response, "Hello, Alice! Welcome to the Internet Computer!");
    
    // Test greeting with empty name
    let response: String = update(&pic, canister_id, "greet", encode_one("").unwrap());
    assert_eq!(response, "Hello, Anonymous!");
}

/// Test the canister info functionality
#[test]
fn test_canister_info() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    let info: CanisterInfo = query(&pic, canister_id, "get_info", encode_args(()).unwrap());
    
    assert_eq!(info.name, "Hello Canister");
    assert_eq!(info.version, "0.1.0");
    assert_eq!(info.description, "A simple hello world canister demonstrating best practices");
}

/// Test the counter functionality
#[test]
fn test_counter_functionality() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Initial counter should be 0
    let initial_count: u64 = query(&pic, canister_id, "get_counter", encode_args(()).unwrap());
    assert_eq!(initial_count, 0);
    
    // Increment counter
    let count1: u64 = update(&pic, canister_id, "increment_counter", encode_args(()).unwrap());
    assert_eq!(count1, 1);
    
    // Check counter again
    let current_count: u64 = query(&pic, canister_id, "get_counter", encode_args(()).unwrap());
    assert_eq!(current_count, 1);
    
    // Increment again
    let count2: u64 = update(&pic, canister_id, "increment_counter", encode_args(()).unwrap());
    assert_eq!(count2, 2);
    
    // Final check
    let final_count: u64 = query(&pic, canister_id, "get_counter", encode_args(()).unwrap());
    assert_eq!(final_count, 2);
}

/// Test multiple interactions in sequence
#[test]
fn test_multiple_interactions() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Greet different users
    let greeting1: String = update(&pic, canister_id, "greet", encode_one("Bob").unwrap());
    let greeting2: String = update(&pic, canister_id, "greet", encode_one("Charlie").unwrap());
    
    assert_eq!(greeting1, "Hello, Bob! Welcome to the Internet Computer!");
    assert_eq!(greeting2, "Hello, Charlie! Welcome to the Internet Computer!");
    
    // Increment counter multiple times
    let _: u64 = update(&pic, canister_id, "increment_counter", encode_args(()).unwrap());
    let _: u64 = update(&pic, canister_id, "increment_counter", encode_args(()).unwrap());
    let count: u64 = update(&pic, canister_id, "increment_counter", encode_args(()).unwrap());
    
    assert_eq!(count, 3);
    
    // Verify final state
    let final_count: u64 = query(&pic, canister_id, "get_counter", encode_args(()).unwrap());
    assert_eq!(final_count, 3);
}

/// Test error handling with invalid inputs
#[test]
fn test_edge_cases() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Test with very long name
    let long_name = "A".repeat(1000);
    let response: String = update(&pic, canister_id, "greet", encode_one(long_name.clone()).unwrap());
    assert!(response.contains(&long_name));
    assert!(response.contains("Hello"));
    
    // Test with special characters
    let special_name = "Héllo Wørld! 🌍";
    let response: String = update(&pic, canister_id, "greet", encode_one(special_name).unwrap());
    assert!(response.contains("Héllo Wørld! 🌍"));
}

// Helper functions

fn setup_pocket_ic() -> PocketIc {
    PocketIcBuilder::new()
        .with_application_subnet()
        .build()
}

fn deploy_hello_canister(pic: &PocketIc) -> Principal {
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);
    
    pic.install_canister(canister_id, HELLO_CANISTER_WASM.to_vec(), vec![], None);
    
    // Allow canister to initialize
    for _ in 0..5 {
        pic.tick();
    }
    
    canister_id
}

/// Generic update call helper
fn update<T: CandidType + for<'de> Deserialize<'de>>(
    pic: &PocketIc,
    canister_id: Principal,
    method: &str,
    args: Vec<u8>,
) -> T {
    let result = pic.update_call(canister_id, Principal::anonymous(), method, args);
    
    match result {
        Ok(reply) => decode_one(&reply).expect("Failed to decode update response"),
        Err(user_error) => panic!("Update call failed: {}", user_error),
    }
}

/// Generic query call helper  
fn query<T: CandidType + for<'de> Deserialize<'de>>(
    pic: &PocketIc,
    canister_id: Principal,
    method: &str,
    args: Vec<u8>,
) -> T {
    let result = pic.query_call(canister_id, Principal::anonymous(), method, args);
    
    match result {
        Ok(reply) => decode_one(&reply).expect("Failed to decode query response"),
        Err(user_error) => panic!("Query call failed: {}", user_error),
    }
}
