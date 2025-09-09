use candid::{CandidType, Deserialize, Principal, Encode, Decode};
use ic_agent::{Agent, Identity};
use ic_agent::identity::{BasicIdentity, Secp256k1Identity};
use sha2::{Sha224, Sha256, Digest};
use std::path::PathBuf;

// Canonical canister IDs
const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const NNS_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

pub struct StakeNeuronArgs {
    pub identity_path: PathBuf,
    pub ic_url: String,
    pub amount: u64,
    pub nonce: u64,
}

pub async fn stake_neuron(args: StakeNeuronArgs) -> Result<NeuronStakingResult, Box<dyn std::error::Error>> {
    println!("Loading identity from: {}", args.identity_path.display());
    println!("Connecting to IC at: {}", args.ic_url);
    println!("Staking amount: {} e8s ({} ICP)", args.amount, args.amount as f64 / 100_000_000.0);
    println!("Using nonce: {}", args.nonce);

    let identity = load_identity(&args.identity_path).await?;

    let controller = identity.sender()?;
    println!("Identity principal: {}", controller);

    let agent = Agent::builder()
        .with_url(&args.ic_url)
        .with_identity(identity)
        .build()
        .expect("Failed to create IC agent");

    agent.fetch_root_key().await?;

    println!("Successfully created IC agent!");

    // Step 1: Compute neuron staking subaccount
    let subaccount = compute_neuron_staking_subaccount(controller, args.nonce);
    println!("Calculated subaccount: \"{}\"", hex::encode(subaccount));

    // Step 2: Transfer ICP to governance canister subaccount
    println!("\n===========TRANSFERRING ICP TO GOVERNANCE=========");
    let governance_principal = Principal::from_text(NNS_GOVERNANCE_CANISTER_ID)?;

    let to_account = AccountIdentifier::new(&governance_principal, Some(subaccount));
    let transfer_args = TransferArgs {
        memo: args.nonce,
        amount: Tokens { e8s: args.amount },
        fee: Tokens { e8s: 10_000 }, // Standard ICP transfer fee
        from_subaccount: None,
        to: to_account.to_vec(),
        created_at_time: None,
    };

    let transfer_result_bytes = agent.update(&Principal::from_text(ICP_LEDGER_CANISTER_ID)?, "transfer")
        .with_arg(Encode!(&transfer_args)?)
        .call_and_wait()
        .await?;

    let transfer_result = Decode!(&transfer_result_bytes, TransferResult)?;

    let block_index = match transfer_result {
        TransferResult::Ok(block_index) => {
            println!("Transfer successful! Block index: {}", block_index);
            block_index
        }
        TransferResult::Err(transfer_error) => {
            println!("Transfer failed: {:?}", transfer_error);
            return Err(format!("Transfer failed: {:?}", transfer_error).into());
        }
    };

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

    let neuron_id = if let Some(result) = claim_result.result {
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
                neuron_id.id
            }
        }
    } else {
        println!("No result received - this might indicate a decoding issue");
        return Err("No result received from governance canister".into());
    };

    println!("\nNeuron staking completed successfully!");

    Ok(NeuronStakingResult {
        neuron_id,
        controller,
        staked_amount_e8s: args.amount,
        block_index,
        subaccount: hex::encode(subaccount),
    })
}

pub struct NeuronStakingResult {
    pub neuron_id: u64,
    pub controller: Principal,
    pub staked_amount_e8s: u64,
    pub block_index: u64,
    pub subaccount: String,
}

async fn load_identity(identity_path: &PathBuf) -> Result<Box<dyn Identity>, Box<dyn std::error::Error>> {
    println!("Attempting to load identity from: {}", identity_path.display());

    // Read the PEM file
    let pem_content = std::fs::read_to_string(identity_path)?;

    // Try to parse as different identity types
    if let Ok(identity) = BasicIdentity::from_pem(pem_content.as_bytes()) {
        println!("Loaded basic identity");
        return Ok(Box::new(identity));
    }

    if let Ok(identity) = Secp256k1Identity::from_pem(pem_content.as_bytes()) {
        println!("Loaded secp256k1 identity");
        return Ok(Box::new(identity));
    }

    Err("Could not parse identity file as either BasicIdentity or Secp256k1Identity".into())
}

/// Compute the subaccount for neuron staking
fn compute_neuron_staking_subaccount(controller: Principal, nonce: u64) -> [u8; 32] {
    let domain_length: [u8; 1] = [b"neuron-stake".len() as u8];
    let mut hasher = Sha256::new();
    hasher.update(&domain_length);
    hasher.update(b"neuron-stake");
    hasher.update(controller.as_slice());
    hasher.update(&nonce.to_be_bytes());
    hasher.finalize().into()
}

// Candid types - all the types we defined manually
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
enum TransferError {
    BadFee { expected_fee: Tokens },
    InsufficientFunds { balance: Tokens },
    TxTooOld { allowed_window_nanos: u64 },
    TxCreatedInFuture,
    TxDuplicate { duplicate_of: u64 },
}

#[derive(CandidType, Deserialize, Debug)]
enum TransferResult {
    Ok(u64),
    Err(TransferError),
}

#[derive(CandidType, Deserialize, Debug)]
struct NeuronId {
    id: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct ClaimOrRefreshNeuronFromAccount {
    controller: Option<Principal>,
    memo: u64,
}

#[derive(CandidType, Deserialize, Debug)]
enum ClaimOrRefreshResult {
    Error(GovernanceError),
    NeuronId(NeuronId),
}

#[derive(CandidType, Deserialize, Debug)]
struct ClaimOrRefreshNeuronFromAccountResponse {
    result: Option<ClaimOrRefreshResult>,
}

#[derive(CandidType, Deserialize, Debug)]
struct GovernanceError {
    error_type: i32,
    error_message: String,
}
