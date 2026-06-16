use candid::{CandidType, Deserialize, Principal};
use ic_cdk::call::Call;
use ic_cdk::{query, update};
use sha2::{Digest, Sha256};

// Canonical mainnet canister IDs for ICP Ledger and NNS Governance
const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const NNS_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

// Standard ICP transfer fee: 0.0001 ICP = 10,000 e8s
const DEFAULT_FEE_E8S: u64 = 10_000;

// ── Candid types for ICP Ledger ─────────────────────────────────────────────

#[derive(CandidType, Deserialize, Clone, Debug)]
struct AccountIdentifier {
    hash: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct Tokens {
    e8s: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct Memo(u64);

#[derive(CandidType, Deserialize, Clone, Debug)]
struct TransferArgs {
    memo: Memo,
    amount: Tokens,
    fee: Tokens,
    from_subaccount: Option<Vec<u8>>,
    to: AccountIdentifier,
    created_at_time: Option<u64>,
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

/// Compute the subaccount for neuron staking.
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
fn compute_neuron_staking_subaccount(controller: Principal, nonce: u64) -> [u8; 32] {
    let domain = b"neuron-stake";
    let domain_length: [u8; 1] = [domain.len() as u8];
    let mut hasher = Sha256::new();
    hasher.update(domain_length);
    hasher.update(domain);
    hasher.update(controller.as_slice());
    hasher.update(nonce.to_be_bytes());
    hasher.finalize().into()
}

/// Build an ICP AccountIdentifier from a principal + subaccount.
///
/// Format: CRC32(0x0a || "account-id" || principal || subaccount)
///         || 0x0a || "account-id" || principal || subaccount
/// The final identifier is the last 28 bytes of the 32-byte SHA-256 hash,
/// prepended with the 4-byte CRC32 checksum.
fn build_account_identifier(principal: Principal, subaccount: &[u8; 32]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(b"\x0aaccount-id");
    hasher.update(principal.as_slice());
    hasher.update(subaccount);
    let hash: [u8; 32] = hasher.finalize().into();

    // CRC32 over the 28-byte account body (bytes 4..32 of hash)
    let crc = crc32(&hash[4..]);
    let mut result = Vec::with_capacity(32);
    result.extend_from_slice(&crc.to_be_bytes());
    result.extend_from_slice(&hash[4..]);
    result
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// ── Canister endpoints ───────────────────────────────────────────────────────

/// Return the hex-encoded neuron staking subaccount for the given
/// (controller, nonce) pair.  Call this before sending any ICP to verify
/// the destination subaccount matches your expectation.
#[query]
fn compute_subaccount(controller: Principal, nonce: u64) -> String {
    let sub = compute_neuron_staking_subaccount(controller, nonce);
    hex_encode(&sub)
}

/// Perform the two-step neuron staking process on behalf of this canister:
///
/// 1. Transfer `amount` e8s of ICP from this canister's default account to
///    the NNS Governance canister at the computed staking subaccount.
/// 2. Call `claim_or_refresh_neuron_from_account` on the Governance canister
///    to complete the neuron creation.
///
/// **Prerequisites (mainnet):**
/// - This canister must hold at least `amount + 10_000` e8s of ICP
///   (the extra 10_000 e8s cover the standard transfer fee).
/// - The minimum staking amount enforced by NNS Governance is 100_000_000 e8s
///   (1 ICP).
///
/// **Local testing:** The NNS canisters are not deployed on a default local
/// network, so this method can only be exercised against mainnet or a local
/// NNS replica.  See the README for details.
#[update]
async fn stake_neuron(amount: u64, nonce: u64) -> Result<StakeNeuronResult, String> {
    let controller = ic_cdk::api::msg_caller();
    let canister_id = ic_cdk::api::canister_self();

    let subaccount = compute_neuron_staking_subaccount(controller, nonce);
    let governance_principal =
        Principal::from_text(NNS_GOVERNANCE_CANISTER_ID).map_err(|e| e.to_string())?;

    let to_account = AccountIdentifier {
        hash: build_account_identifier(governance_principal, &subaccount),
    };

    // Step 1: Transfer ICP to the governance staking subaccount
    let transfer_args = TransferArgs {
        memo: Memo(nonce),
        amount: Tokens { e8s: amount },
        fee: Tokens {
            e8s: DEFAULT_FEE_E8S,
        },
        from_subaccount: None,
        to: to_account,
        created_at_time: None,
    };

    let ledger =
        Principal::from_text(ICP_LEDGER_CANISTER_ID).map_err(|e| e.to_string())?;

    let transfer_result: TransferResult = Call::bounded_wait(ledger, "transfer")
        .with_arg(transfer_args)
        .await
        .map_err(|e| format!("Transfer call failed: {:?}", e))?
        .candid()
        .map_err(|e| format!("Transfer decode failed: {:?}", e))?;

    let block_index = match transfer_result {
        TransferResult::Ok(idx) => idx,
        TransferResult::Err(e) => return Err(format!("Transfer failed: {:?}", e)),
    };

    // Step 2: Claim the neuron using the same nonce
    let claim_request = ClaimOrRefreshNeuronFromAccount {
        controller: Some(canister_id),
        memo: nonce,
    };

    let claim_response: ClaimOrRefreshNeuronFromAccountResponse =
        Call::bounded_wait(governance_principal, "claim_or_refresh_neuron_from_account")
            .with_arg(claim_request)
            .await
            .map_err(|e| format!("Governance call failed: {:?}", e))?
            .candid()
            .map_err(|e| format!("Governance decode failed: {:?}", e))?;

    let neuron_id = match claim_response.result {
        Some(ClaimOrRefreshResult::NeuronId(n)) => n.id,
        Some(ClaimOrRefreshResult::Error(e)) => {
            return Err(format!("Governance error ({}): {}", e.error_type, e.error_message))
        }
        None => return Err("No result returned from governance canister".to_string()),
    };

    Ok(StakeNeuronResult {
        neuron_id,
        controller,
        staked_amount_e8s: amount,
        block_index,
        subaccount_hex: hex_encode(&subaccount),
    })
}

// Export Candid interface
ic_cdk::export_candid!();
