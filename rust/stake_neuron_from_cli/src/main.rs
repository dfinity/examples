use clap::Parser;
use ic_agent::{Agent, Identity};
use ic_agent::identity::{BasicIdentity, Secp256k1Identity};
use std::path::PathBuf;
use std::io::Cursor;
use candid::{CandidType, Deserialize, Principal, Encode, Decode};
use sha2::{Sha224, Sha256, Digest};

// Canonical canister IDs
const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const NNS_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the private key file (PEM format)
    #[arg(short, long)]
    identity: PathBuf,

    /// URL of the IC replica
    #[arg(short, long, default_value = "http://127.0.0.1:4943")]
    url: String,
    
    /// Amount of ICP to stake (in e8s, default: 1 ICP + fees = 100_010_000 e8s)
    #[arg(short, long, default_value = "100010000")]
    amount: u64,
    
    /// Nonce for neuron creation (default: 0)
    #[arg(short, long, default_value = "0")]
    nonce: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Loading identity from: {}", args.identity.display());
    println!("Connecting to IC at: {}", args.url);
    println!("Staking amount: {} e8s ({} ICP)", args.amount, args.amount as f64 / 100_000_000.0);
    println!("Using nonce: {}", args.nonce);

    let identity = load_identity(&args.identity).await?;
    let controller = identity.sender()?;
    
    println!("Identity principal: {}", controller);

    let agent = Agent::builder()
        .with_url(&args.url)
        .with_identity(identity)
        .build()
        .expect("Failed to create IC agent");

    agent.fetch_root_key().await?;
    
    println!("Successfully created IC agent!");

    // Step 1: Calculate neuron staking subaccount
    let subaccount = compute_neuron_staking_subaccount_bytes(&controller, args.nonce);
    println!("Calculated subaccount: {:?}", hex::encode(&subaccount));

    // Step 2: Transfer ICP to governance canister subaccount
    println!("\n===========TRANSFERRING ICP TO GOVERNANCE=========");
    let governance_principal = Principal::from_text(NNS_GOVERNANCE_CANISTER_ID)?;
    
    let to_account = AccountIdentifier::new(&governance_principal, Some(subaccount));
    let transfer_args = TransferArgs {
        memo: args.nonce,
        amount: Tokens { e8s: args.amount },
        fee: Tokens { e8s: 10_000 }, // Standard ICP transfer fee
        from_subaccount: None,
        // NOTE: See `to_vec` implementation, as it gives a 32-bit address with necessary checksum
        // instead of the 28-bit address. The ledger requires the 32-bit address.
        to: to_account.to_vec(),
        created_at_time: None,
    };

    let transfer_result_bytes = agent.update(&Principal::from_text(ICP_LEDGER_CANISTER_ID)?, "transfer")
        .with_arg(Encode!(&transfer_args)?)
        .call_and_wait()
        .await?;
    
    let transfer_result = Decode!(&transfer_result_bytes, TransferResult)?;

    match transfer_result {
        TransferResult::Ok(block_index) => {
            println!("Transfer successful! Block index: {}", block_index);
        }
        TransferResult::Err(transfer_error) => {
            println!("Transfer failed: {:?}", transfer_error);
            return Err(format!("Transfer failed: {:?}", transfer_error).into());
        }
    }

    // Step 3: Claim or refresh neuron from account
    println!("\n===========CLAIMING NEURON FROM ACCOUNT=========");
    
    let claim_request = ClaimOrRefreshNeuronFromAccount {
        controller: Some(controller),
        memo: args.nonce,
    };

    let claim_result_bytes = agent.update(&governance_principal, "claim_or_refresh_neuron_from_account")
        .with_arg(Encode!(&claim_request)?)
        .call_and_wait()
        .await?;
    
    println!("Raw response bytes length: {}", claim_result_bytes.len());
    println!("Raw response bytes (hex): {}", hex::encode(&claim_result_bytes));
    
    let claim_result = Decode!(&claim_result_bytes, ClaimOrRefreshNeuronFromAccountResponse)?;

    println!("Full governance response: {:?}", claim_result);

    if let Some(result) = claim_result.result {
        match result {
            ClaimOrRefreshResult::Error(error) => {
                println!("Governance error: {} (type: {})", error.error_message, error.error_type);
                return Err(format!("Governance error: {}", error.error_message).into());
            }
            ClaimOrRefreshResult::NeuronId(neuron_id) => {
                println!("Neuron successfully claimed/refreshed!");
                println!("Neuron ID: {}", neuron_id.id);
                println!("Controller: {}", controller);
                println!("Staked amount: {} e8s ({} ICP)", args.amount, args.amount as f64 / 100_000_000.0);
            }
        }
    } else {
        println!("No result received - this might indicate a decoding issue");
    }

    println!("\nNeuron staking completed successfully!");
    Ok(())
}

async fn load_identity(path: &PathBuf) -> Result<Box<dyn Identity>, Box<dyn std::error::Error>> {
    println!("Attempting to load identity from: {}", path.display());
    
    if !path.exists() {
        return Err(format!("Identity file not found: {}", path.display()).into());
    }

    let key_content = std::fs::read_to_string(path)?;
    
    if let Ok(identity) = BasicIdentity::from_pem(Cursor::new(key_content.as_bytes())) {
        println!("Loaded Ed25519 identity");
        return Ok(Box::new(identity));
    }
    
    if let Ok(identity) = Secp256k1Identity::from_pem(Cursor::new(key_content.as_bytes())) {
        println!("Loaded secp256k1 identity");
        return Ok(Box::new(identity));
    }
    
    Err("Failed to parse identity file as either Ed25519 or secp256k1".into())
}

/// Compute the subaccount for neuron staking
/// NOTE: This is a key part of staking the neuron subaccount.  If this algorithm chooses the
/// wrong subaccount, you will not make the transfer to the correct place.  Test your implementation
/// using something like the example in this repository to ensure that it can be successfully
/// executed.
fn compute_neuron_staking_subaccount_bytes(controller: &Principal, nonce: u64) -> [u8; 32] {
    let domain_length: [u8; 1] = [b"neuron-stake".len() as u8];
    let mut hasher = Sha256::new();
    hasher.update(&domain_length);
    hasher.update(b"neuron-stake");
    hasher.update(controller.as_slice());
    hasher.update(&nonce.to_be_bytes());
    hasher.finalize().into()
}

// ============================================================================
// Type Definitions
//
// NOTE: In production code, using didc or some other code generation to translate
// the declared types of the canister into Rust is a better practice.  However, for the
// sake of this example, we simply copy the types from the dfinity/ic repository.
// ============================================================================

#[derive(CandidType, Deserialize, Debug)]
struct Tokens {
    e8s: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct AccountIdentifier {
    hash: [u8; 28],
}

impl AccountIdentifier {
    fn new(principal: &Principal, subaccount: Option<[u8; 32]>) -> Self {
        let mut hasher = Sha224::new();
        hasher.update(b"\x0Aaccount-id");
        hasher.update(principal.as_slice());
        
        let sub_account = subaccount.unwrap_or([0u8; 32]);
        hasher.update(&sub_account);
        
        AccountIdentifier {
            hash: hasher.finalize().into(),
        }
    }
    
    fn to_vec(&self) -> Vec<u8> {
        let checksum = crc32fast::hash(&self.hash);
        let checksum_bytes = checksum.to_be_bytes();
        [&checksum_bytes[..], &self.hash[..]].concat()
    }
}

#[derive(CandidType, Deserialize, Debug)]
struct TransferArgs {
    memo: u64,
    amount: Tokens,
    fee: Tokens,
    from_subaccount: Option<Vec<u8>>,
    to: Vec<u8>,
    created_at_time: Option<TimeStamp>,
}

#[derive(CandidType, Deserialize, Debug)]
struct TimeStamp {
    timestamp_nanos: u64,
}

#[derive(CandidType, Deserialize, Debug)]
enum TransferResult {
    Ok(u64), // Block index
    Err(TransferError),
}

#[derive(CandidType, Deserialize, Debug)]
enum TransferError {
    BadFee { expected_fee: Tokens },
    InsufficientFunds { balance: Tokens },
    TxTooOld { allowed_window_nanos: u64 },
    TxCreatedInFuture,
    TxDuplicate { duplicate_of: u64 },
}

#[derive(CandidType, Deserialize, Debug)]
struct NeuronId {
    id: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct ManageNeuron {
    id: Option<NeuronId>,
    command: Option<Command>,
    neuron_id_or_subaccount: Option<NeuronIdOrSubaccount>,
}

#[derive(CandidType, Deserialize, Debug)]
enum NeuronIdOrSubaccount {
    #[serde(with = "serde_bytes")]
    Subaccount(Vec<u8>),
    NeuronId(NeuronId),
}

#[derive(CandidType, Deserialize, Debug)]
enum Command {
    ClaimOrRefresh(ClaimOrRefreshNeuronFromAccount),
}

#[derive(CandidType, Deserialize, Debug)]
struct ClaimOrRefreshNeuronFromAccount {
    controller: Option<Principal>,
    memo: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct ClaimOrRefreshNeuronFromAccountResponse {
    result: Option<ClaimOrRefreshResult>,
}

#[derive(CandidType, Deserialize, Debug)]
enum ClaimOrRefreshResult {
    Error(GovernanceError),
    NeuronId(NeuronId),
}

#[derive(CandidType, Deserialize, Debug)]
struct ManageNeuronResponse {
    command: Option<CommandResponse>,
}

#[derive(CandidType, Deserialize, Debug)]
struct GovernanceError {
    error_type: i32,
    error_message: String,
}

#[derive(CandidType, Deserialize, Debug)]
enum CommandResponse {
    Error(GovernanceError),
    ClaimOrRefresh(ClaimOrRefreshResponse),
    Configure(ConfigureResponse),
    Disburse(DisburseResponse),
    Spawn(SpawnResponse),
    Follow(FollowResponse),
    MakeProposal(MakeProposalResponse),
    RegisterVote(RegisterVoteResponse),
    Split(SplitResponse),
    DisburseToNeuron(DisburseToNeuronResponse),
    MergeMaturity(MergeMaturityResponse),
    Merge(MergeResponse),
    StakeMaturity(StakeMaturityResponse),
    RefreshVotingPower(RefreshVotingPowerResponse),
    DisburseMaturity(DisburseMaturityResponse),
    SetFollowing(SetFollowingResponse),
}

#[derive(CandidType, Deserialize, Debug)]
struct ConfigureResponse {}

#[derive(CandidType, Deserialize, Debug)]
struct DissolveResponse {}

#[derive(CandidType, Deserialize, Debug)]
struct FollowResponse {}

#[derive(CandidType, Deserialize, Debug)]
struct MakeProposalResponse {
    proposal_id: Option<ProposalId>,
}

#[derive(CandidType, Deserialize, Debug)]
struct ProposalId {
    id: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct RegisterVoteResponse {}

#[derive(CandidType, Deserialize, Debug)]
struct SplitResponse {
    created_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize, Debug)]
struct SpawnResponse {
    created_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize, Debug)]
struct MergeResponse {
    target_neuron: Option<Neuron>,
    source_neuron: Option<Neuron>,
    target_neuron_info: Option<NeuronInfo>,
    source_neuron_info: Option<NeuronInfo>,
}

#[derive(CandidType, Deserialize, Debug)]
struct DisburseResponse {
    transfer_block_height: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct MergeMaturityResponse {
    merged_maturity_e8s: u64,
    new_stake_e8s: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct DisburseMaturityResponse {
    amount_disbursed_e8s: u64,
    amount_deducted_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
struct DisburseToNeuronResponse {
    created_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize, Debug)]
struct StakeMaturityResponse {
    maturity_e8s: u64,
    staked_maturity_e8s: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct RefreshVotingPowerResponse {}

#[derive(CandidType, Deserialize, Debug)]
struct SetFollowingResponse {}

#[derive(CandidType, Deserialize, Debug)]
struct Neuron {
    id: Option<NeuronId>,
    // Add other fields as needed for debugging
}

#[derive(CandidType, Deserialize, Debug)]
struct NeuronInfo {
    // Add fields as needed for debugging
}

#[derive(CandidType, Deserialize, Debug)]
struct ClaimOrRefreshResponse {
    refreshed_neuron_id: Option<NeuronId>,
}