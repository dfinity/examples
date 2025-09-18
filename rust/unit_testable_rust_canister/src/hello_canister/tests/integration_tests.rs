use candid::{decode_one, encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, PocketIcBuilder};
use serde::Deserialize;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use walkdir::WalkDir;

// Import all request/response types from the library
use hello_canister::types::nns_governance::{
    Followees, Governance, NetworkEconomics, NeuronId, Proposal, ProposalData, ProposalId,
};
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
        println!("Rebuilding wasm under test");
        rebuild_wasm();
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

fn download_wasm_to(url: String, wasm_path: &Path) {
    // Ensure the target directory exists
    if let Some(parent) = wasm_path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|e| panic!("Failed to create directory {:?}: {}", parent, e));
    }

    let wasm = std::process::Command::new("curl")
        .args(["-L", "-f", &url])
        .output()
        .expect("Failed to download NNS Governance WASM")
        .stdout;

    std::fs::write(wasm_path, wasm).expect("Failed to write compressed WASM");
}

/// Get the NNS Governance WASM binary, downloading if necessary
fn get_governance_wasm() -> Vec<u8> {
    let wasm_path = PathBuf::from("../../target/ic/governance-canister.wasm.gz");

    // Check if we need to download
    if !wasm_path.exists() {
        let url = format!(
            "https://download.dfinity.systems/ic/{}/canisters/governance-canister.wasm.gz",
            IC_COMMIT_FOR_PROPOSALS
        );
        download_wasm_to(url, &wasm_path);
    } else {
        println!("NNS Governance WASM already exists, skipping download");
    }

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

    // Create the canister with the specific mainnet ID and NNS root as controller
    pic.create_canister_with_id(Some(nns_root_canister_id), None, governance_canister_id)
        .expect("Failed to create governance canister with specific ID");

    // Use the governance types directly from our types.rs
    let init_args = default_governance_for_tests();

    println!("Installing NNS Governance canister with production-like configuration");

    // Install the canister with the initialization arguments
    pic.install_canister(
        governance_canister_id,
        governance_wasm,
        encode_one(init_args).expect("Failed to encode governance init args"),
        Some(nns_root_canister_id),
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
    let request = GetCountRequest {};
    let response: GetCountResponse =
        query(&pic, canister_id, "get_count", encode_one(request).unwrap());
    assert_eq!(response.count, Some(0));

    // Increment counter
    let request = IncrementCountRequest {};
    let response: IncrementCountResponse = update(
        &pic,
        canister_id,
        "increment_count",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.new_count, Some(1));

    // Check counter again
    let request = GetCountRequest {};
    let response: GetCountResponse =
        query(&pic, canister_id, "get_count", encode_one(request).unwrap());
    assert_eq!(response.count, Some(1));

    // Increment again
    let request = IncrementCountRequest {};
    let response: IncrementCountResponse = update(
        &pic,
        canister_id,
        "increment_count",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.new_count, Some(2));

    // Final check
    let request = GetCountRequest {};
    let response: GetCountResponse =
        query(&pic, canister_id, "get_count", encode_one(request).unwrap());
    assert_eq!(response.count, Some(2));

    // Increment counter
    let request = IncrementCountRequest {};
    let response: IncrementCountResponse = update(
        &pic,
        canister_id,
        "decrement_count",
        encode_one(request).unwrap(),
    );
    assert_eq!(response.new_count, Some(1));
}

/// Test async governance functionality - List Proposals
/// Note: It is difficult to set up NNS Governance in integration tests from
/// outside the IC monorepo.  One of the principal purposes of this repository is to show
/// how much can be tested in unit tests via mocking, so that business logic can be fully vetted
/// without needing to set up mainnet canisters with correct configurations in your tests.
#[test]
fn test_get_proposal_titles() {
    let pic = setup_pocket_ic();
    setup_nns_governance(&pic);

    let canister_id = deploy_hello_canister(&pic);

    // Test listing proposals (should return mock data)
    let request = GetProposalTitlesRequest { limit: None };
    let response: GetProposalTitlesResponse = update(
        &pic,
        canister_id,
        "get_proposal_titles",
        encode_one(request).unwrap(),
    );

    let titles = response.titles.unwrap();

    assert_eq!(titles.len(), 10);

    // Test listing proposals (should return mock data)
    let request = GetProposalTitlesRequest { limit: Some(20) };
    let response: GetProposalTitlesResponse = update(
        &pic,
        canister_id,
        "get_proposal_titles",
        encode_one(request).unwrap(),
    );

    let titles = response.titles.unwrap();

    assert_eq!(titles.len(), 20);

    // Test listing proposals (should return mock data)
    let request = GetProposalTitlesRequest { limit: Some(1) };
    let response: GetProposalTitlesResponse = update(
        &pic,
        canister_id,
        "get_proposal_titles",
        encode_one(request).unwrap(),
    );

    let titles = response.titles.unwrap();

    assert_eq!(titles.len(), 1);
}

/// Test async governance functionality - Get Proposal Info
#[test]
fn test_get_proposal_info() {
    let pic = setup_pocket_ic();
    let canister_id = deploy_hello_canister(&pic);
    setup_nns_governance(&pic);

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

    assert_eq!(response.error, None);

    let info = response.basic_info.unwrap();
    assert_eq!(info.title.unwrap(), "Test Title 1");

    // Test with no proposal ID (should return error)
    let request = GetProposalInfoRequest { proposal_id: None };
    let response: GetProposalInfoResponse = update(
        &pic,
        canister_id,
        "get_proposal_info",
        encode_one(request).unwrap(),
    );

    assert!(response.basic_info.is_none());
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap(), "Missing proposal_id");
}

// Helper functions

fn setup_pocket_ic() -> PocketIc {
    PocketIcBuilder::new()
        .with_application_subnet()
        .with_nns_subnet()
        .build()
}

fn deploy_hello_canister(pic: &PocketIc) -> Principal {
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    // Use smart WASM rebuilding - will only rebuild if source files changed
    let wasm_binary = get_hello_canister_wasm();

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

/// Helper function that provides a pre-configured Governance instance
/// This uses the same defaults that GovernanceCanisterInitPayloadBuilder::new().build() would provide
pub fn default_governance_for_tests() -> Governance {
    // Create default followees with empty lists for key topics
    // In production, these would point to foundation neurons
    let mut default_followees = vec![];

    for topic in 1..18 {
        default_followees.push((
            topic,
            Followees {
                followees: vec![], // Empty for testing - production would have foundation neuron IDs
            },
        ));
    }

    let economics = NetworkEconomics {
        neuron_minimum_stake_e8s: 100_000_000, // 1 ICP
        voting_power_economics: None,
        max_proposals_to_keep_per_topic: 100,
        neuron_management_fee_per_proposal_e8s: 1_000_000, // 0.01 ICP
        reject_cost_e8s: 1_000_000,                        // 0.01 ICP
        transaction_fee_e8s: 10_000,                       // 0.0001 ICP
        neuron_spawn_dissolve_delay_seconds: 7 * 24 * 60 * 60, // 7 days
        minimum_icp_xdr_rate: 100,
        maximum_node_provider_rewards_e8s: 1_000_000_000_000, // 10,000 ICP
        neurons_fund_economics: None,
    };

    Governance {
        economics: Some(economics),
        restore_aging_summary: None,
        spawning_neurons: None,
        latest_reward_event: None,
        default_followees,
        making_sns_proposal: None,
        most_recent_monthly_node_provider_rewards: None,
        maturity_modulation_last_updated_at_timestamp_seconds: None,
        wait_for_quiet_threshold_seconds: 86400, // 24 hours (production-like)
        short_voting_period_seconds: 345600,     // 4 days (production-like)
        proposals: (1..21)
            .map(|id| {
                (
                    id,
                    ProposalData {
                        id: Some(ProposalId { id }),
                        proposer: Some(NeuronId { id: 123 }),
                        proposal_timestamp_seconds: 1640000000,
                        reward_event_round: 0,
                        failed_timestamp_seconds: 0,
                        reject_cost_e8s: 0,
                        derived_proposal_information: None,
                        latest_tally: None,
                        total_potential_voting_power: None,
                        decided_timestamp_seconds: 0,
                        topic: Some(13),
                        failure_reason: None,
                        ballots: vec![],
                        proposal: Some(Box::from(Proposal {
                            url: "".to_string(),
                            title: Some(format!("Test Title {id}")),
                            action: None,
                            summary: "".to_string(),
                        })),
                        executed_timestamp_seconds: 0,
                        neurons_fund_data: None,
                        sns_token_swap_lifecycle: None,
                        wait_for_quiet_state: None,
                        original_total_community_fund_maturity_e8s_equivalent: None,
                    },
                )
            })
            .collect(),
        xdr_conversion_rate: None,
        in_flight_commands: vec![],
        neuron_management_voting_period_seconds: Some(172800), // 2 days for neuron management
        node_providers: vec![],
        genesis_timestamp_seconds: 1620000000, // Realistic timestamp (May 2021)
        metrics: None,
        cached_daily_maturity_modulation_basis_points: None,
        to_claim_transfers: vec![],
        neurons: vec![],
    }
}
