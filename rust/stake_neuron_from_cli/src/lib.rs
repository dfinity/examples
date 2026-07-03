use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_agent::{
    identity::{BasicIdentity, Secp256k1Identity},
    Agent, Identity,
};
use ic_ledger_types::{
    AccountIdentifier, Memo, Subaccount, Tokens, TransferArgs, TransferResult,
    MAINNET_LEDGER_CANISTER_ID,
};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const NNS_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

pub struct StakeNeuronArgs {
    pub identity_path: PathBuf,
    pub ic_url: String,
    pub amount_e8s: u64,
    pub nonce: u64,
}

pub async fn stake_neuron(args: StakeNeuronArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.amount_e8s < 100_000_000 {
        return Err("amount_e8s must be at least 100_000_000 (1 ICP)".into());
    }

    let pem = std::fs::read(&args.identity_path)?;
    // Try Ed25519 first, then secp256k1 (default key type created by icp-cli).
    let identity: Box<dyn Identity> = if let Ok(id) = BasicIdentity::from_pem(&pem) {
        Box::new(id)
    } else {
        Box::new(Secp256k1Identity::from_pem(&pem)?)
    };
    let controller = identity.sender()?;

    let agent = Agent::builder()
        .with_url(&args.ic_url)
        .with_identity(identity)
        .build()?;
    // fetch_root_key is only needed for local networks; on mainnet the root key is
    // compiled into ic-agent and this call is a no-op.
    agent.fetch_root_key().await?;

    println!("Controller : {controller}");
    println!("Amount     : {} e8s ({} ICP)", args.amount_e8s, args.amount_e8s as f64 / 1e8);
    println!("Nonce      : {}", args.nonce);

    // ── Step 1: transfer ICP to governance's staking subaccount ─────────────

    let new_neuron_subaccount = compute_neuron_staking_subaccount(controller, args.nonce);
    println!("Subaccount : {}", hex_encode(&new_neuron_subaccount.0));

    let governance = Principal::from_text(NNS_GOVERNANCE_CANISTER_ID).unwrap();
    let transfer_args = TransferArgs {
        // The memo here is only a label on the ledger transaction; it does not need
        // to match the nonce passed to claim_or_refresh_neuron_from_account below.
        memo: Memo(args.nonce),
        amount: Tokens::from_e8s(args.amount_e8s),
        fee: Tokens::from_e8s(10_000),
        from_subaccount: None,
        to: AccountIdentifier::new(&governance, &new_neuron_subaccount),
        created_at_time: None,
    };

    let transfer_bytes = agent
        .update(&MAINNET_LEDGER_CANISTER_ID, "transfer")
        .with_arg(Encode!(&transfer_args)?)
        .call_and_wait()
        .await?;
    let block_index = match Decode!(&transfer_bytes, TransferResult)? {
        TransferResult::Ok(idx) => {
            println!("Transfer OK (block {idx})");
            idx
        }
        TransferResult::Err(e) => return Err(format!("Transfer failed: {e:?}").into()),
    };

    // ── Step 2: claim the neuron ─────────────────────────────────────────────
    // Anyone can call claim_or_refresh_neuron_from_account to complete this step
    // because the neuron's controller is already determined by the subaccount
    // derived in step 1 — regardless of who the caller is here.

    let claim_request = ClaimOrRefreshNeuronFromAccount {
        // The controller that will own the neuron; Governance recomputes
        // the expected subaccount as hash(controller, memo) and checks funds.
        controller: Some(controller),
        // This memo IS the nonce used when deriving the staking subaccount above
        // and must match it exactly.
        memo: args.nonce,
    };

    let claim_bytes = agent
        .update(&governance, "claim_or_refresh_neuron_from_account")
        .with_arg(Encode!(&claim_request)?)
        .call_and_wait()
        .await?;
    let response = Decode!(&claim_bytes, ClaimOrRefreshNeuronFromAccountResponse)?;

    let neuron_id = match response.result {
        Some(ClaimOrRefreshResult::NeuronId(n)) => n.id,
        Some(ClaimOrRefreshResult::Error(e)) => {
            return Err(format!("Governance error ({}): {}", e.error_type, e.error_message).into())
        }
        None => return Err("No result from governance canister".into()),
    };

    println!("Neuron ID  : {neuron_id}");
    println!("Block index: {block_index}");
    Ok(())
}

/// Compute the NNS neuron staking subaccount for a (controller, nonce) pair.
///
/// The neuron staking subaccount is the most critical thing to get right.
/// If ICP is sent to the wrong subaccount it is permanently lost.
///
/// SHA-256 over these inputs in order:
///   1. 12  (the length of the domain separator "neuron-stake")
///   2. "neuron-stake"
///   3. controller principal as raw bytes (not the text form)
///   4. nonce as big-endian u64
pub fn compute_neuron_staking_subaccount(controller: Principal, nonce: u64) -> Subaccount {
    let domain = b"neuron-stake";
    let mut hasher = Sha256::new();
    hasher.update([domain.len() as u8]);
    hasher.update(domain);
    hasher.update(controller.as_slice());
    hasher.update(nonce.to_be_bytes());
    Subaccount(hasher.finalize().into())
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// ── Candid types for NNS Governance ─────────────────────────────────────────

#[derive(CandidType, Deserialize, Debug)]
struct ClaimOrRefreshNeuronFromAccount {
    controller: Option<Principal>,
    memo: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct NeuronId {
    id: u64,
}

#[derive(CandidType, Deserialize, Debug)]
struct GovernanceError {
    error_type: i32,
    error_message: String,
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
