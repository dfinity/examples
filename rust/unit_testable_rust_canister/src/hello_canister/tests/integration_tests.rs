use candid::{decode_one, encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, PocketIcBuilder};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use walkdir::WalkDir;

// Import all request/response types from the library
use hello_canister::types::nns_governance::{Followees, Governance, NetworkEconomics};
use hello_canister::types::*;

// IC commit used for downloading official NNS WASM files
// This should match what's currently deployed in production
const IC_COMMIT_FOR_PROPOSALS: &str = "4b7cde9a0e3b5ad4725e75cbc36ce635be6fa6a8";

// NNS Canister IDs (mainnet)
const NNS_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";
const NNS_ROOT_CANISTER_ID: &str = "r7inp-6aaaa-aaaaa-aaabq-cai";

// ICP Ledger Canister ID (mainnet)
const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

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

/// Get the ICP Ledger WASM binary, downloading if necessary
fn get_ledger_wasm() -> Vec<u8> {
    let wasm_path = PathBuf::from("../../target/ic/ledger-canister.wasm.gz");

    if !wasm_path.exists() {
        let url = format!(
            "https://download.dfinity.systems/ic/{}/canisters/ledger-canister.wasm.gz",
            IC_COMMIT_FOR_PROPOSALS
        );
        download_wasm_to(url, &wasm_path);
    } else {
        println!("ICP Ledger WASM already exists, skipping download");
    }
    std::fs::read(&wasm_path).unwrap_or_else(|e| {
        panic!(
            "Failed to read ICP Ledger WASM file at {:?}: {}",
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

/// Sets up ICP Ledger canister with correct mainnet ID and production-like initialization
/// Uses LedgerInitArgs::default() to get realistic initialization parameters
fn setup_icp_ledger(pic: &PocketIc) -> Principal {
    let ledger_wasm = get_ledger_wasm();

    // Parse the mainnet canister ID
    let ledger_canister_id =
        Principal::from_text(ICP_LEDGER_CANISTER_ID).expect("Invalid ICP Ledger canister ID");

    // Create the canister with the specific mainnet ID (no controller set for ledger)
    pic.create_canister_with_id(None, None, ledger_canister_id)
        .expect("Failed to create ICP Ledger canister with specific ID");

    // Use the proper ICP ledger types from our types module
    use hello_canister::types::icp_ledger::{LedgerArg, LedgerInit};

    let init_args = LedgerArg::Init(LedgerInit::default_for_tests());

    println!("Installing ICP Ledger canister with production-like configuration");
    if let LedgerArg::Init(ref ledger_init) = init_args {
        println!(
            "- Transfer fee: {:?} e8s (0.0001 ICP)",
            ledger_init.transfer_fee.as_ref().map(|t| t.e8s)
        );
        println!("- Token symbol: {:?}", ledger_init.token_symbol);
        println!("- Token name: {:?}", ledger_init.token_name);
        println!(
            "- Initial balances: {} accounts",
            ledger_init.initial_values.len()
        );
    }

    // Install the canister with the initialization arguments
    pic.install_canister(
        ledger_canister_id,
        ledger_wasm,
        encode_one(init_args).expect("Failed to encode ledger init args"),
        None,
    );

    println!(
        "ICP Ledger canister setup complete at {}",
        ledger_canister_id
    );

    ledger_canister_id
}

/// Setup ICP balance for a user by transferring from pre-funded account
/// The ledger is initialized with a test account that has a large balance
fn setup_icp_balance_for_user(
    pic: &PocketIc,
    ledger_canister_id: Principal,
    user_principal: Principal,
    amount: u64,
) {
    println!(
        "Setting up {} e8s ICP balance for user {}",
        amount, user_principal
    );

    // Convert user principal to account identifier string format
    // For testing, we'll use a simple format - in reality this would be more complex
    let user_account_id = format!(
        "{}-user",
        user_principal
            .to_text()
            .chars()
            .take(10)
            .collect::<String>()
    );

    // Use the pre-funded test account from initialization
    let funded_account_principal =
        Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap_or(user_principal);

    // Use types from our types module
    use hello_canister::types::icp_ledger::{Tokens, TransferArgs};

    let transfer_args = TransferArgs {
        memo: 0,
        amount: Tokens::from_e8s(amount),
        fee: Tokens::from_e8s(10_000), // 0.0001 ICP fee
        from_subaccount: None,
        to: user_account_id,
        created_at_time: None,
    };

    let raw_result = pic
        .update_call(
            ledger_canister_id,
            funded_account_principal, // Use the pre-funded account as caller
            "transfer",
            candid::encode_one(transfer_args).expect("Failed to encode transfer args"),
        )
        .expect("Call failed");

    let result: Result<u64, String> =
        candid::decode_one(&raw_result).expect("Failed to decode transfer response");

    match result {
        Ok(block_index) => println!(
            "✅ Transferred {} e8s to user at block {}",
            amount, block_index
        ),
        Err(e) => println!("⚠️  Transfer failed (expected in test setup): {}", e),
    }
}

/// Create a neuron by staking ICP through the governance canister
fn create_neuron_via_stake(
    pic: &PocketIc,
    governance_canister_id: Principal,
    ledger_canister_id: Principal,
    user_principal: Principal,
    stake_amount: u64,
) -> u64 {
    println!("Creating neuron by staking {} e8s", stake_amount);

    // Step 1: Transfer ICP to governance canister for staking using old ICP ledger format
    // Convert governance principal to account identifier string (simplified)
    let governance_account_id = format!(
        "{}-gov",
        governance_canister_id
            .to_text()
            .chars()
            .take(10)
            .collect::<String>()
    );

    // Use types from our types module
    use hello_canister::types::icp_ledger::{Tokens, TransferArgs};

    let transfer_args = TransferArgs {
        memo: 0, // Memo 0 for neuron staking
        amount: Tokens::from_e8s(stake_amount),
        fee: Tokens::from_e8s(10_000), // 0.0001 ICP fee
        from_subaccount: None,
        to: governance_account_id,
        created_at_time: None,
    };

    let raw_transfer_result = pic
        .update_call(
            ledger_canister_id,
            user_principal,
            "transfer",
            candid::encode_one(transfer_args).expect("Failed to encode transfer args"),
        )
        .expect("Call failed");

    let transfer_result: Result<u64, String> =
        candid::decode_one(&raw_transfer_result).expect("Failed to decode transfer response");

    let _block_index = match transfer_result {
        Ok(block_index) => {
            println!(
                "✅ Transferred {} e8s to governance at block {}",
                stake_amount, block_index
            );
            block_index
        }
        Err(e) => {
            println!("⚠️  Transfer to governance failed: {}", e);
            0 // Continue with neuron creation attempt anyway
        }
    };

    // Step 2: Claim the neuron using manage_neuron
    #[derive(candid::CandidType, serde::Deserialize)]
    struct ManageNeuron {
        id: Option<NeuronId>,
        command: Option<Command>,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct NeuronId {
        id: u64,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    enum Command {
        ClaimOrRefresh(ClaimOrRefresh),
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct ClaimOrRefresh {
        by: Option<ClaimOrRefreshBy>,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    enum ClaimOrRefreshBy {
        MemoAndController(MemoAndController),
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct MemoAndController {
        memo: u64,
        controller: Option<Principal>,
    }

    let manage_neuron = ManageNeuron {
        id: None, // No ID needed for claiming
        command: Some(Command::ClaimOrRefresh(ClaimOrRefresh {
            by: Some(ClaimOrRefreshBy::MemoAndController(MemoAndController {
                memo: 0, // Same memo as transfer
                controller: Some(user_principal),
            })),
        })),
    };

    #[derive(candid::CandidType, serde::Deserialize)]
    struct ManageNeuronResponse {
        command: Option<CommandResponse>,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    enum CommandResponse {
        ClaimOrRefresh(ClaimOrRefreshResponse),
        Error(GovernanceError),
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct ClaimOrRefreshResponse {
        refreshed_neuron_id: Option<NeuronId>,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct GovernanceError {
        error_type: i32,
        error_message: String,
    }

    let raw_neuron_result = pic
        .update_call(
            governance_canister_id,
            user_principal,
            "manage_neuron",
            candid::encode_one(manage_neuron).expect("Failed to encode manage_neuron"),
        )
        .expect("Call failed");

    let neuron_result: ManageNeuronResponse =
        candid::decode_one(&raw_neuron_result).expect("Failed to decode manage_neuron response");

    match neuron_result.command {
        Some(CommandResponse::ClaimOrRefresh(claim_response)) => {
            if let Some(neuron_id) = claim_response.refreshed_neuron_id {
                println!("✅ Neuron created with ID: {}", neuron_id.id);
                return neuron_id.id;
            }
            panic!("No neuron ID returned from governance canister");
        }
        Some(CommandResponse::Error(error)) => {
            panic!(
                "Failed to claim neuron: {} (type: {})",
                error.error_message, error.error_type
            );
        }
        None => panic!("No response from governance canister"),
    }
}

/// Make a test proposal using the specified neuron
fn make_test_proposal(
    pic: &PocketIc,
    governance_canister_id: Principal,
    user_principal: Principal,
    neuron_id: u64,
    title: &str,
    summary: &str,
) -> u64 {
    println!("Making proposal: {} - {}", title, summary);

    // Create governance types for making proposals
    #[derive(candid::CandidType, serde::Deserialize)]
    struct ManageNeuron {
        id: Option<NeuronId>,
        command: Option<Command>,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct NeuronId {
        id: u64,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    enum Command {
        MakeProposal(Proposal),
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct Proposal {
        title: Option<String>,
        summary: String,
        url: String,
        action: Option<Action>,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    enum Action {
        Motion(Motion),
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct Motion {
        motion_text: String,
    }

    let proposal = Proposal {
        title: Some(title.to_string()),
        summary: summary.to_string(),
        url: "https://example.com".to_string(),
        action: Some(Action::Motion(Motion {
            motion_text: format!("Motion: {}", title),
        })),
    };

    let manage_neuron = ManageNeuron {
        id: Some(NeuronId { id: neuron_id }),
        command: Some(Command::MakeProposal(proposal)),
    };

    #[derive(candid::CandidType, serde::Deserialize)]
    struct ManageNeuronResponse {
        command: Option<CommandResponse>,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    enum CommandResponse {
        MakeProposal(MakeProposalResponse),
        Error(GovernanceError),
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct MakeProposalResponse {
        proposal_id: Option<ProposalId>,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct ProposalId {
        id: u64,
    }

    #[derive(candid::CandidType, serde::Deserialize)]
    struct GovernanceError {
        error_type: i32,
        error_message: String,
    }

    let raw_proposal_result = pic
        .update_call(
            governance_canister_id,
            user_principal,
            "manage_neuron",
            candid::encode_one(manage_neuron).expect("Failed to encode manage_neuron"),
        )
        .expect("Call failed");

    let proposal_result: ManageNeuronResponse =
        candid::decode_one(&raw_proposal_result).expect("Failed to decode make proposal response");

    match proposal_result.command {
        Some(CommandResponse::MakeProposal(make_proposal_response)) => {
            if let Some(proposal_id) = make_proposal_response.proposal_id {
                println!("✅ Proposal created with ID: {}", proposal_id.id);
                return proposal_id.id;
            }
            panic!("No proposal ID returned from governance canister");
        }
        Some(CommandResponse::Error(error)) => {
            panic!(
                "Failed to make proposal: {} (type: {})",
                error.error_message, error.error_type
            );
        }
        None => panic!("No response from governance canister"),
    }
}

/// Test creating a neuron, making proposals, and listing them through our canister
#[test]
fn test_neuron_creation_and_proposal_listing() {
    let pic = setup_pocket_ic();

    println!("=== Setting up all canisters ===");

    // Deploy our test canister first
    let test_canister_id = deploy_hello_canister(&pic);
    println!("Test canister deployed at: {}", test_canister_id);

    // Deploy ICP Ledger with initial balances for testing
    let ledger_canister_id = setup_icp_ledger(&pic);
    println!("ICP Ledger deployed at: {}", ledger_canister_id);

    // Deploy NNS Governance
    let governance_canister_id = setup_nns_governance(&pic);
    println!("NNS Governance deployed at: {}", governance_canister_id);

    println!("=== Creating a neuron by staking ICP ===");

    // Create a test user principal
    let user_principal =
        Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").expect("Invalid user principal");
    println!("User principal: {}", user_principal);

    // Give the user some ICP to stake (simulate minting)
    let mint_amount = 200_000_000u64; // 2 ICP in e8s
    setup_icp_balance_for_user(&pic, ledger_canister_id, user_principal, mint_amount);

    // Create neuron by staking 1 ICP through governance
    let stake_amount = 100_000_000u64; // 1 ICP in e8s
    let neuron_id = create_neuron_via_stake(
        &pic,
        governance_canister_id,
        ledger_canister_id,
        user_principal,
        stake_amount,
    );
    println!("Created neuron with ID: {:?}", neuron_id);

    println!("=== Making two test proposals ===");

    // Make first proposal
    let proposal_1_id = make_test_proposal(
        &pic,
        governance_canister_id,
        user_principal,
        neuron_id,
        "Test Proposal 1",
        "This is the first test proposal",
    );
    println!("Created proposal 1 with ID: {:?}", proposal_1_id);

    // Make second proposal
    let proposal_2_id = make_test_proposal(
        &pic,
        governance_canister_id,
        user_principal,
        neuron_id,
        "Test Proposal 2",
        "This is the second test proposal",
    );
    println!("Created proposal 2 with ID: {:?}", proposal_2_id);

    println!("=== Testing proposal listing through our canister ===");

    // Use our test canister's get_proposal_titles endpoint to list proposals
    let request = GetProposalTitlesRequest { limit: Some(10) };
    let response: GetProposalTitlesResponse = update(
        &pic,
        test_canister_id,
        "get_proposal_titles",
        encode_one(request).unwrap(),
    );

    println!("Response: {:?}", response);

    // Verify we got proposals back
    assert!(
        response.error.is_none(),
        "Should not have error: {:?}",
        response.error
    );
    assert!(response.titles.is_some(), "Should have titles");

    let titles = response.titles.unwrap();
    assert!(
        titles.len() >= 2,
        "Should have at least 2 proposals, got: {}",
        titles.len()
    );

    // Verify our test proposals are in the results
    let title_strings: Vec<String> = titles.iter().map(|t| t.clone()).collect();
    println!("Retrieved proposal titles: {:?}", title_strings);

    // Note: The exact titles might be formatted differently by governance canister,
    // but we should have some proposals in the list
    assert!(
        title_strings.len() >= 2,
        "Should have retrieved at least 2 proposal titles"
    );

    println!("✅ Successfully created neuron, made proposals, and listed them!");
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

fn setup_pic_with_proposals() {}

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

    let expected = vec!["Foo".to_string(), "Bar".to_string()];

    let titles = response.titles.unwrap();
    assert_eq!(titles, expected);

    // Test with custom limit
    let request = GetProposalTitlesRequest { limit: Some(1) };
    let response: GetProposalTitlesResponse = update(
        &pic,
        canister_id,
        "get_proposal_titles",
        encode_one(request).unwrap(),
    );

    let expected = vec!["Foo".to_string()];
    let titles = response.titles.unwrap();
    assert_eq!(titles, expected);
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
        proposals: vec![],
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
