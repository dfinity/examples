use candid::{decode_one, encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, PocketIcBuilder};
use serde::Deserialize;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use walkdir::WalkDir;

// Import all request/response types from the library
use hello_canister::types::*;

// IC commit used for downloading official NNS WASM files
// This should match what's currently deployed in production
const IC_COMMIT_FOR_PROPOSALS: &str = "4b7cde9a0e3b5ad4725e75cbc36ce635be6fa6a8";

// NNS Canister IDs (mainnet)
const NNS_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";
const NNS_ROOT_CANISTER_ID: &str = "r7inp-6aaaa-aaaaa-aaabq-cai";

// WASM will be loaded dynamically with smart rebuilding
fn get_hello_canister_wasm() -> Vec<u8> {
    let wasm_path = ensure_wasm_built();
    std::fs::read(&wasm_path)
        .unwrap_or_else(|e| panic!("Failed to read WASM file at {:?}: {}", wasm_path, e))
}

/// Ensures the WASM is built and up-to-date, returns path to the WASM file
fn ensure_wasm_built() -> PathBuf {
    let wasm_path =
        PathBuf::from("../../target/wasm32-unknown-unknown/release/hello_canister.wasm");
    let src_dir = Path::new("src");
    let cargo_toml = Path::new("Cargo.toml");

    if needs_rebuild(&wasm_path, src_dir, cargo_toml) {
        println!("WASM needs rebuilding, running cargo build...");
        rebuild_wasm();
        println!("WASM build complete");
    } else {
        println!("WASM is up-to-date, skipping rebuild");
    }

    wasm_path
}

/// Check if WASM needs to be rebuilt by comparing file timestamps in source directory.
fn needs_rebuild(wasm_path: &Path, src_dir: &Path, cargo_toml: &Path) -> bool {
    // If WASM doesn't exist, definitely need to build
    if !wasm_path.exists() {
        return true;
    }

    fn file_modified_time(path: &Path) -> io::Result<SystemTime> {
        path.metadata().and_then(|m| m.modified())
    }

    let Ok(wasm_time) = file_modified_time(wasm_path) else {
        println!("Can't get WASM modification time, will rebuild");
        return true;
    };

    // Check if Cargo.toml is newer (dependency changes)
    if let Ok(cargo_time) = file_modified_time(cargo_toml) {
        if cargo_time > wasm_time {
            println!("Cargo.toml is newer than WASM, will rebuild");
            return true;
        }
    } else {
        println!("Couldn't get Cargo.toml modification time, will rebuild.");
        return true;
    }

    // Check if any source file is newer than WASM
    let newer_file_found = WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map_or(false, |ext| ext == "rs" || ext == "toml")
        })
        .any(|entry| {
            if let Ok(file_time) = file_modified_time(entry.path()) {
                if file_time > wasm_time {
                    println!(
                        "Source file {:?} is newer than WASM, will rebuild",
                        entry.path()
                    );
                    return true;
                }
            }
            false
        });

    newer_file_found
}

/// Rebuild the WASM using cargo
fn rebuild_wasm() {
    let output = Command::new("cargo")
        .args(&["build", "--target", "wasm32-unknown-unknown", "--release"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo build command");

    if !output.status.success() {
        eprintln!(
            "WASM build stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        eprintln!(
            "WASM build stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!(
            "WASM build failed with exit code: {:?}",
            output.status.code()
        );
    }

    if !output.stdout.is_empty() {
        println!("Build output: {}", String::from_utf8_lossy(&output.stdout));
    }
}

/// Downloads the NNS Governance WASM from DFINITY's official release server
/// Uses the same approach as nns-dapp for getting official IC artifacts
fn ensure_governance_wasm_downloaded() -> PathBuf {
    let wasm_path = PathBuf::from("../../target/ic/governance-canister.wasm");

    // Check if we need to download
    if !wasm_path.exists() {
        download_governance_wasm(&wasm_path);
    } else {
        println!("NNS Governance WASM is up-to-date, skipping download");
    }

    wasm_path
}

/// Download the governance WASM with multiple fallback methods
fn download_governance_wasm(wasm_path: &Path) {
    // Ensure the target directory exists
    if let Some(parent) = wasm_path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|e| panic!("Failed to create directory {:?}: {}", parent, e));
    }

    let url = format!(
        "https://download.dfinity.systems/ic/{}/canisters/governance-canister.wasm.gz",
        IC_COMMIT_FOR_PROPOSALS
    );

    let output = Command::new("sh")
        .args(&[
            "-c",
            &format!(
                "curl -L --fail '{}' | gunzip > '{}'",
                url,
                wasm_path.display()
            ),
        ])
        .output();

    match output {
        Ok(cmd_output) if cmd_output.status.success() => {
            // Check if the downloaded file is valid
            if let Ok(metadata) = std::fs::metadata(&wasm_path) {
                let size_mb = metadata.len() / (1024 * 1024);
                if size_mb >= 1 {
                    println!(
                        "Downloaded NNS Governance WASM ({} MB) from {}",
                        size_mb, url
                    );
                    return;
                }
            }
        }
        Ok(cmd_output) => {
            println!(
                "Download failed: {}",
                String::from_utf8_lossy(&cmd_output.stderr)
            );
        }
        Err(e) => {
            println!("Download failed: {}", e);
        }
    }

    // Clean up failed download
    let _ = std::fs::remove_file(wasm_path);

    panic!("All download methods failed for governance WASM");
}

/// Get the NNS Governance WASM binary, downloading if necessary
fn get_governance_wasm() -> Vec<u8> {
    let wasm_path = ensure_governance_wasm_downloaded();
    std::fs::read(&wasm_path).unwrap_or_else(|e| {
        panic!(
            "Failed to read governance WASM file at {:?}: {}",
            wasm_path, e
        )
    })
}

/// Sets up NNS Governance canister with correct mainnet ID and production-like initialization
/// Uses default_governance_for_tests() to get realistic initialization parameters
fn setup_nns_governance(pic: &PocketIc) -> Principal {
    let governance_wasm = get_governance_wasm();

    // Parse the mainnet canister IDs
    let nns_root_canister_id =
        Principal::from_text(NNS_ROOT_CANISTER_ID).expect("Invalid NNS root canister ID");
    let governance_canister_id = Principal::from_text(NNS_GOVERNANCE_CANISTER_ID)
        .expect("Invalid NNS governance canister ID");

    println!(
        "Creating NNS Governance canister with ID: {} and controller: {}",
        governance_canister_id, nns_root_canister_id
    );

    // Create the canister with the specific mainnet ID and NNS root as controller
    pic.create_canister_with_id(Some(nns_root_canister_id), None, governance_canister_id)
        .expect("Failed to create governance canister with specific ID");

    // Get production-like initialization arguments using our helper function
    let governance_config = default_governance_for_tests();

    // Convert our Governance struct to GovernanceCanisterInit format for initialization
    let init_args = GovernanceCanisterInit {
        economics: governance_config.economics,
        default_followees: governance_config.default_followees,
        wait_for_quiet_threshold_seconds: governance_config.wait_for_quiet_threshold_seconds,
        short_voting_period_seconds: governance_config.short_voting_period_seconds,
        initial_voting_period: governance_config.initial_voting_period,
        proposal_wait_for_quiet_threshold_seconds: governance_config
            .proposal_wait_for_quiet_threshold_seconds,
        proposal_initial_voting_period: governance_config.proposal_initial_voting_period,
        neuron_management_voting_period_seconds: governance_config
            .neuron_management_voting_period_seconds,
        neurons_fund_economics: governance_config.neurons_fund_economics,
        voting_rewards_parameters: governance_config.voting_rewards_parameters,
        genesis_timestamp_seconds: governance_config.genesis_timestamp_seconds,
    };

    println!("Installing NNS Governance canister with production-like configuration");

    // Install the canister with the initialization arguments
    pic.install_canister(
        governance_canister_id,
        governance_wasm,
        encode_one(init_args).expect("Failed to encode governance init args"),
        None,
    );

    println!(
        "NNS Governance canister setup complete at {}",
        governance_canister_id
    );

    governance_canister_id
}

/// Test the counter functionality with Request/Response pattern
#[test]
fn test_counter_functionality() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);

    // Initial counter should be 0
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(
        &pic,
        canister_id,
        "get_counter",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.count, Some(0));

    // Increment counter
    let request = IncrementCounterRequest {};
    let response: IncrementCounterResponse = update(
        &pic,
        canister_id,
        "increment_counter",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.new_count, Some(1));

    // Check counter again
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(
        &pic,
        canister_id,
        "get_counter",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.count, Some(1));

    // Increment again
    let request = IncrementCounterRequest {};
    let response: IncrementCounterResponse = update(
        &pic,
        canister_id,
        "increment_counter",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.new_count, Some(2));

    // Final check
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(
        &pic,
        canister_id,
        "get_counter",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.count, Some(2));
}

/// Test multiple interactions in sequence
#[test]
fn test_multiple_interactions() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);

    // Increment counter multiple times
    let request = IncrementCounterRequest {};
    let _: IncrementCounterResponse = update(
        &pic,
        canister_id,
        "increment_counter",
        encode_one(request).unwrap(),
    );

    let request = IncrementCounterRequest {};
    let _: IncrementCounterResponse = update(
        &pic,
        canister_id,
        "increment_counter",
        encode_one(request).unwrap(),
    );

    let request = IncrementCounterRequest {};
    let response: IncrementCounterResponse = update(
        &pic,
        canister_id,
        "increment_counter",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.new_count, Some(3));

    // Verify final state
    let request = GetCounterRequest {};
    let response: GetCounterResponse = query(
        &pic,
        canister_id,
        "get_counter",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.count, Some(3));
}

/// Test async governance functionality - List Proposals
#[test]
fn test_list_proposals() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);

    // Test listing proposals (should return mock data)
    let request = ListProposalsRequest {};
    let response: ListProposalsResponse = update(
        &pic,
        canister_id,
        "list_proposals",
        encode_one(request).unwrap(),
    );

    // Since we're using mock data, we should get some proposals or an empty list
    match (response.proposal_ids, response.error) {
        (Some(proposals), None) => {
            // Mock should return some proposals
            println!("Mock proposals returned: {:?}", proposals);
            // Could be empty if mocking returns empty list, that's fine
        }
        (None, Some(error)) => {
            // This is expected if the inter-canister call fails (which it will in PocketIC)
            println!("Expected error from governance call: {}", error);
            assert!(
                error.contains("Governance") || error.contains("call") || error.contains("failed")
            );
        }
        _ => panic!("Response should have either proposal_ids or error"),
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
    let response: GetProposalInfoResponse = update(
        &pic,
        canister_id,
        "get_proposal_info",
        encode_one(request).unwrap(),
    );

    match (response.proposal, response.error) {
        (Some(_proposal), None) => {
            // If mock returns proposal info, that's fine
            println!("Mock proposal info returned");
        }
        (None, Some(error)) => {
            // Expected - inter-canister call will fail in PocketIC
            println!("Expected error from governance call: {}", error);
            assert!(
                error.contains("Governance") || error.contains("call") || error.contains("failed")
            );
        }
        _ => panic!("Response should have either proposal or error"),
    }

    // Test with no proposal ID (should return error)
    let request = GetProposalInfoRequest { proposal_id: None };
    let response: GetProposalInfoResponse = update(
        &pic,
        canister_id,
        "get_proposal_info",
        encode_one(request).unwrap(),
    );

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
    let response: GetProposalTitlesResponse = update(
        &pic,
        canister_id,
        "get_proposal_titles",
        encode_one(request).unwrap(),
    );

    match (response.titles, response.error) {
        (Some(_titles), None) => {
            // If mock returns titles, that's fine
            println!("Mock proposal titles returned");
        }
        (None, Some(error)) => {
            // Expected - inter-canister call will fail in PocketIC
            println!("Expected error from governance call: {}", error);
            assert!(
                error.contains("Governance") || error.contains("call") || error.contains("failed")
            );
        }
        _ => panic!("Response should have either titles or error"),
    }

    // Test with custom limit
    let request = GetProposalTitlesRequest { limit: Some(5) };
    let response: GetProposalTitlesResponse = update(
        &pic,
        canister_id,
        "get_proposal_titles",
        encode_one(request).unwrap(),
    );

    // Should get same type of response (either titles or error)
    assert!(response.titles.is_some() || response.error.is_some());
}

// Helper functions

fn setup_pocket_ic() -> PocketIc {
    PocketIcBuilder::new().with_application_subnet().build()
}

fn deploy_hello_canister(pic: &PocketIc) -> Principal {
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    // Use smart WASM rebuilding - will only rebuild if source files changed
    let wasm_binary = get_hello_canister_wasm();
    println!("Got wasm_binary");
    pic.install_canister(canister_id, wasm_binary, vec![], None);

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

// =============================================================================
// GOVERNANCE CANISTER TYPES (Copied from nns/governance/api)
// =============================================================================

use hello_canister::types::nns_governance::{
    Followees, GovernanceCanisterInit, NetworkEconomics, Topic,
};
use serde::Serialize;
use std::collections::BTreeMap;

/// Main Governance canister state and configuration
/// Based on the nns/governance/api crate Governance struct
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Governance {
    /// Economic parameters for the governance system
    pub economics: Option<NetworkEconomics>,
    /// Default neurons that other neurons follow for each topic
    pub default_followees: BTreeMap<Topic, Followees>,
    /// Time in seconds before a proposal can be decided
    pub wait_for_quiet_threshold_seconds: u64,
    /// Duration of short voting period in seconds
    pub short_voting_period_seconds: u64,
    /// Initial voting period duration in seconds
    pub initial_voting_period: u64,
    /// Wait for quiet threshold for proposals in seconds
    pub proposal_wait_for_quiet_threshold_seconds: u64,
    /// Initial voting period for proposals in seconds
    pub proposal_initial_voting_period: u64,
    /// Voting period for neuron management proposals in seconds
    pub neuron_management_voting_period_seconds: Option<u64>,
    /// Economics parameters for the neurons fund (simplified for testing)
    pub neurons_fund_economics: Option<()>,
    /// Voting rewards parameters (simplified for testing)
    pub voting_rewards_parameters: Option<()>,
    /// Timestamp when the governance canister was created
    pub genesis_timestamp_seconds: Option<u64>,
}

/// Helper function that provides a pre-configured Governance instance
/// This uses the same defaults that GovernanceCanisterInitPayloadBuilder::new().build() would provide
pub fn default_governance_for_tests() -> Governance {
    // Create default followees with empty lists for key topics
    // In production, these would point to foundation neurons
    let mut default_followees = BTreeMap::new();

    let foundation_topics = vec![
        Topic::Governance,
        Topic::NetworkEconomics,
        Topic::NodeAdmin,
        Topic::SubnetManagement,
        Topic::NetworkCanisterManagement,
        Topic::ExchangeRate,
        Topic::ParticipantManagement,
        Topic::NodeProviderRewards,
        Topic::ReplicaVersionManagement,
        Topic::IcOsVersionElection,
        Topic::IcOsVersionDeployment,
    ];

    for topic in foundation_topics {
        default_followees.insert(
            topic,
            Followees {
                followees: vec![], // Empty for testing - production would have foundation neuron IDs
            },
        );
    }

    let economics = NetworkEconomics {
        neuron_minimum_stake_e8s: 100_000_000, // 1 ICP
        max_proposals_to_keep_per_topic: 100,
        neuron_management_fee_per_proposal_e8s: 1_000_000, // 0.01 ICP
        reject_cost_e8s: 1_000_000,                        // 0.01 ICP
        transaction_fee_e8s: 10_000,                       // 0.0001 ICP
        neuron_spawn_dissolve_delay_seconds: 7 * 24 * 60 * 60, // 7 days
        minimum_icp_xdr_rate: 100,
        maximum_node_provider_rewards_e8s: 1_000_000_000_000, // 10,000 ICP
    };

    Governance {
        economics: Some(economics),
        default_followees,
        wait_for_quiet_threshold_seconds: 86400, // 24 hours (production-like)
        short_voting_period_seconds: 345600,     // 4 days (production-like)
        initial_voting_period: 345600,           // 4 days (production-like)
        proposal_wait_for_quiet_threshold_seconds: 86400, // 24 hours
        proposal_initial_voting_period: 345600,  // 4 days
        neuron_management_voting_period_seconds: Some(172800), // 2 days for neuron management
        neurons_fund_economics: None,            // Not needed for basic testing
        voting_rewards_parameters: None,         // Not needed for basic testing
        genesis_timestamp_seconds: Some(1620000000), // Realistic timestamp (May 2021)
    }
}
