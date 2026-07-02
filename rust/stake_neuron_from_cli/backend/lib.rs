use candid::{CandidType, Deserialize, Principal};
use ic_cdk::call::Call;
use ic_cdk::{query, update};
use ic_ledger_types::{
    AccountIdentifier, Memo, Subaccount, Tokens, MAINNET_LEDGER_CANISTER_ID,
};
use sha2::{Digest, Sha256};

const NNS_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

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

// ── Public return type ───────────────────────────────────────────────────────

#[derive(CandidType, Deserialize, Debug)]
pub struct StakeNeuronResult {
    pub neuron_id: u64,
    pub controller: Principal,
    pub staked_amount_e8s: u64,
    pub block_index: u64,
    pub subaccount_hex: String,
}

// ── Core logic ───────────────────────────────────────────────────────────────

/// Compute the staking subaccount for a (controller, nonce) pair.
///
/// The neuron staking subaccount is the most critical thing to get right.
/// If ICP is sent to the wrong subaccount it is permanently lost — the
/// Governance canister cannot retrieve it.
///
/// Domain-separated SHA-256 hash over:
///   \x0c  (length of "neuron-stake" = 12)
///   "neuron-stake"
///   controller bytes
///   nonce as big-endian u64
fn compute_neuron_staking_subaccount(controller: Principal, nonce: u64) -> Subaccount {
    let domain = b"neuron-stake";
    let mut hasher = Sha256::new();
    hasher.update([domain.len() as u8]);
    hasher.update(domain);
    hasher.update(controller.as_slice());
    hasher.update(nonce.to_be_bytes());
    Subaccount(hasher.finalize().into())
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// ── Canister endpoints ───────────────────────────────────────────────────────

/// Return the hex-encoded neuron staking subaccount for the given
/// (controller, nonce) pair. Call this before sending any ICP to verify
/// the destination subaccount matches your expectation.
#[query]
fn compute_subaccount(controller: Principal, nonce: u64) -> String {
    hex_encode(&compute_neuron_staking_subaccount(controller, nonce).0)
}

/// Perform the two-step neuron staking process on behalf of this canister:
///
/// 1. Transfer `amount` e8s of ICP from this canister's default account to
///    the NNS Governance canister at the computed staking subaccount.
/// 2. Call `claim_or_refresh_neuron_from_account` on the Governance canister
///    to complete the neuron creation.
///
/// **Prerequisites:**
/// - The canister must hold at least `amount + 10_000` e8s of ICP
///   (10_000 e8s covers the standard transfer fee).
/// - The minimum staking amount enforced by NNS Governance is 100_000_000 e8s (1 ICP).
/// - The local network must be started with `nns: true` in `icp.yaml` so that the
///   ICP Ledger and NNS Governance canisters are available at their mainnet IDs.
#[update]
async fn stake_neuron(amount: u64, nonce: u64) -> Result<StakeNeuronResult, String> {
    let canister_id = ic_cdk::api::canister_self();

    // The canister holds the ICP and becomes the neuron controller, so the
    // staking subaccount is derived from the canister's own principal.
    let subaccount = compute_neuron_staking_subaccount(canister_id, nonce);
    let governance_principal =
        Principal::from_text(NNS_GOVERNANCE_CANISTER_ID).map_err(|e| e.to_string())?;

    // Step 1: Transfer ICP to the Governance canister's staking subaccount.
    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(nonce),
        amount: Tokens::from_e8s(amount),
        fee: Tokens::from_e8s(10_000),
        from_subaccount: None,
        to: AccountIdentifier::new(&governance_principal, &subaccount),
        created_at_time: None,
    };

    let block_index = ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, &transfer_args)
        .await
        .map_err(|e| format!("ledger call failed: {:?}", e))?
        .map_err(|e| format!("ledger transfer error: {:?}", e))?;

    // Step 2: Claim the neuron — Governance checks that it received ICP at
    // hash(controller, memo) and mints the neuron.
    let claim_request = ClaimOrRefreshNeuronFromAccount {
        controller: Some(canister_id),
        memo: nonce,
    };

    let claim_response: ClaimOrRefreshNeuronFromAccountResponse =
        Call::bounded_wait(governance_principal, "claim_or_refresh_neuron_from_account")
            .with_arg(claim_request)
            .await
            .map_err(|e| format!("governance call failed: {:?}", e))?
            .candid()
            .map_err(|e| format!("governance decode failed: {:?}", e))?;

    let neuron_id = match claim_response.result {
        Some(ClaimOrRefreshResult::NeuronId(n)) => n.id,
        Some(ClaimOrRefreshResult::Error(e)) => {
            return Err(format!("governance error ({}): {}", e.error_type, e.error_message))
        }
        None => return Err("no result returned from governance canister".to_string()),
    };

    Ok(StakeNeuronResult {
        neuron_id,
        controller: canister_id,
        staked_amount_e8s: amount,
        block_index,
        subaccount_hex: hex_encode(&subaccount.0),
    })
}

// Export Candid interface
ic_cdk::export_candid!();
