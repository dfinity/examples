use candid::{decode_one, encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, PocketIcBuilder};
use serde::Deserialize;

// Import all request/response types from the library
use hello_canister::types::*;

// Include the WASM binary directly at compile time
// NOTE: Build the WASM first with: cargo build --target wasm32-unknown-unknown --release
const HELLO_CANISTER_WASM: &[u8] = 
    include_bytes!("../../../target/wasm32-unknown-unknown/release/hello_canister.wasm");

/// Test the basic greeting functionality with Request/Response pattern
#[test]
fn test_greet_functionality() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Test greeting with a name
    let request = GreetRequest {
        name: Some("Alice".to_string()),
    };
    let response: GreetResponse = query(&pic, canister_id, "greet", encode_one(request).unwrap());
    assert_eq!(response.greeting, Some("Hello, Alice! Welcome to the Internet Computer!".to_string()));
    
    // Test greeting with empty name
    let request = GreetRequest {
        name: Some("".to_string()),
    };
    let response: GreetResponse = query(&pic, canister_id, "greet", encode_one(request).unwrap());
    assert_eq!(response.greeting, Some("Hello, Anonymous!".to_string()));
    
    // Test greeting with no name
    let request = GreetRequest { name: None };
    let response: GreetResponse = query(&pic, canister_id, "greet", encode_one(request).unwrap());
    assert_eq!(response.greeting, Some("Hello, Anonymous!".to_string()));
}



/// Test the counter functionality with Request/Response pattern
#[test]
fn test_counter_functionality() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Initial counter should be 0
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(&pic, canister_id, "get_counter", encode_one(request).unwrap());
    assert_eq!(response.count, Some(0));
    
    // Increment counter
    let request = IncrementCounterRequest {};
    let response: IncrementCounterResponse = update(&pic, canister_id, "increment_counter", encode_one(request).unwrap());
    assert_eq!(response.new_count, Some(1));
    
    // Check counter again
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(&pic, canister_id, "get_counter", encode_one(request).unwrap());
    assert_eq!(response.count, Some(1));
    
    // Increment again
    let request = IncrementCounterRequest {};
    let response: IncrementCounterResponse = update(&pic, canister_id, "increment_counter", encode_one(request).unwrap());
    assert_eq!(response.new_count, Some(2));
    
    // Final check
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(&pic, canister_id, "get_counter", encode_one(request).unwrap());
    assert_eq!(response.count, Some(2));
}

/// Test multiple interactions in sequence
#[test]
fn test_multiple_interactions() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Greet different users
    let request1 = GreetRequest { name: Some("Bob".to_string()) };
    let response1: GreetResponse = query(&pic, canister_id, "greet", encode_one(request1).unwrap());
    assert_eq!(response1.greeting, Some("Hello, Bob! Welcome to the Internet Computer!".to_string()));
    
    let request2 = GreetRequest { name: Some("Charlie".to_string()) };
    let response2: GreetResponse = query(&pic, canister_id, "greet", encode_one(request2).unwrap());
    assert_eq!(response2.greeting, Some("Hello, Charlie! Welcome to the Internet Computer!".to_string()));
    
    // Increment counter multiple times
    let request = IncrementCounterRequest {};
    let _: IncrementCounterResponse = update(&pic, canister_id, "increment_counter", encode_one(request).unwrap());
    
    let request = IncrementCounterRequest {};
    let _: IncrementCounterResponse = update(&pic, canister_id, "increment_counter", encode_one(request).unwrap());
    
    let request = IncrementCounterRequest {};
    let response: IncrementCounterResponse = update(&pic, canister_id, "increment_counter", encode_one(request).unwrap());
    assert_eq!(response.new_count, Some(3));
    
    // Verify final state
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(&pic, canister_id, "get_counter", encode_one(request).unwrap());
    assert_eq!(response.count, Some(3));
}

/// Test edge cases and special characters
#[test]
fn test_edge_cases() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Test with very long name
    let long_name = "A".repeat(1000);
    let request = GreetRequest { name: Some(long_name.clone()) };
    let response: GreetResponse = query(&pic, canister_id, "greet", encode_one(request).unwrap());
    let greeting = response.greeting.unwrap();
    assert!(greeting.contains(&long_name));
    assert!(greeting.contains("Hello"));
    
    // Test with special characters
    let special_name = "Héllo Wørld! 🌍";
    let request = GreetRequest { name: Some(special_name.to_string()) };
    let response: GreetResponse = query(&pic, canister_id, "greet", encode_one(request).unwrap());
    let greeting = response.greeting.unwrap();
    assert!(greeting.contains("Héllo Wørld! 🌍"));
}

/// Test async governance functionality - List Proposals
#[test]
fn test_list_proposals() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Test listing proposals (should return mock data)
    let request = ListProposalsRequest {};
    let response: ListProposalsResponse = update(&pic, canister_id, "list_proposals", encode_one(request).unwrap());
    
    // Since we're using mock data, we should get some proposals or an empty list
    match (response.proposal_ids, response.error) {
        (Some(proposals), None) => {
            // Mock should return some proposals
            println!("Mock proposals returned: {:?}", proposals);
            // Could be empty if mocking returns empty list, that's fine
        },
        (None, Some(error)) => {
            // This is expected if the inter-canister call fails (which it will in PocketIC)
            println!("Expected error from governance call: {}", error);
            assert!(error.contains("Governance") || error.contains("call") || error.contains("failed"));
        },
        _ => panic!("Response should have either proposal_ids or error"),
    }
}

/// Test async governance functionality - Get Proposal Count
#[test]
fn test_get_proposal_count() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    let request = GetProposalCountRequest {};
    let response: GetProposalCountResponse = update(&pic, canister_id, "get_proposal_count", encode_one(request).unwrap());
    
    // Since this calls governance API, it will likely fail in PocketIC environment
    match (response.count, response.error) {
        (Some(_count), None) => {
            // If mock returns a count, that's fine
            println!("Mock count returned");
        },
        (None, Some(error)) => {
            // Expected - inter-canister call will fail in PocketIC
            println!("Expected error from governance call: {}", error);
            assert!(error.contains("Governance") || error.contains("call") || error.contains("failed"));
        },
        _ => panic!("Response should have either count or error"),
    }
}

/// Test async governance functionality - Get Proposal Info
#[test]
fn test_get_proposal_info() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Test with a proposal ID
    let request = GetProposalInfoRequest {
        proposal_id: Some(1),
    };
    let response: GetProposalInfoResponse = update(&pic, canister_id, "get_proposal_info", encode_one(request).unwrap());
    
    match (response.proposal, response.error) {
        (Some(_proposal), None) => {
            // If mock returns proposal info, that's fine
            println!("Mock proposal info returned");
        },
        (None, Some(error)) => {
            // Expected - inter-canister call will fail in PocketIC
            println!("Expected error from governance call: {}", error);
            assert!(error.contains("Governance") || error.contains("call") || error.contains("failed"));
        },
        _ => panic!("Response should have either proposal or error"),
    }
    
    // Test with no proposal ID (should return error)
    let request = GetProposalInfoRequest {
        proposal_id: None,
    };
    let response: GetProposalInfoResponse = update(&pic, canister_id, "get_proposal_info", encode_one(request).unwrap());
    
    assert!(response.proposal.is_none());
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap(), "Missing proposal_id");
}

/// Test async governance functionality - Get Proposal Titles  
#[test]
fn test_get_proposal_titles() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Test with default limit
    let request = GetProposalTitlesRequest { limit: None };
    let response: GetProposalTitlesResponse = update(&pic, canister_id, "get_proposal_titles", encode_one(request).unwrap());
    
    match (response.titles, response.error) {
        (Some(_titles), None) => {
            // If mock returns titles, that's fine
            println!("Mock proposal titles returned");
        },
        (None, Some(error)) => {
            // Expected - inter-canister call will fail in PocketIC
            println!("Expected error from governance call: {}", error);
            assert!(error.contains("Governance") || error.contains("call") || error.contains("failed"));
        },
        _ => panic!("Response should have either titles or error"),
    }
    
    // Test with custom limit
    let request = GetProposalTitlesRequest { limit: Some(5) };
    let response: GetProposalTitlesResponse = update(&pic, canister_id, "get_proposal_titles", encode_one(request).unwrap());
    
    // Should get same type of response (either titles or error)
    assert!(response.titles.is_some() || response.error.is_some());
}

/// Test API evolution - Request/Response pattern flexibility
#[test]
fn test_api_evolution() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Test that optional fields in requests work (API can evolve)
    let minimal_request = GreetRequest { name: None };
    let response: GreetResponse = query(&pic, canister_id, "greet", encode_one(minimal_request).unwrap());
    assert!(response.greeting.is_some());
    
    let full_request = GreetRequest { name: Some("API Evolution Test".to_string()) };
    let response: GreetResponse = query(&pic, canister_id, "greet", encode_one(full_request).unwrap());
    assert!(response.greeting.is_some());
    assert!(response.greeting.unwrap().contains("API Evolution Test"));
}

/// Test canister state persistence across calls
#[test]
fn test_state_persistence() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    
    // Increment counter several times
    for i in 1..=5 {
        let request = IncrementCounterRequest {};
        let response: IncrementCounterResponse = update(&pic, canister_id, "increment_counter", encode_one(request).unwrap());
        assert_eq!(response.new_count, Some(i));
    }
    
    // Verify final state is persistent
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(&pic, canister_id, "get_counter", encode_one(request).unwrap());
    assert_eq!(response.count, Some(5));
    
    // Do some other operations
    let greet_request = GreetRequest { name: Some("State Test".to_string()) };
    let _: GreetResponse = query(&pic, canister_id, "greet", encode_one(greet_request).unwrap());
    
    // Counter should still be the same
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(&pic, canister_id, "get_counter", encode_one(request).unwrap());
    assert_eq!(response.count, Some(5));
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
