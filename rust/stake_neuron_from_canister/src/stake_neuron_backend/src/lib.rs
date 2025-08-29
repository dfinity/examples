use candid::{CandidType, Principal};
use ic_cdk::{update, query, call};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// NNS Governance Canister ID (local)
const GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";
// ICP Ledger Canister ID (local) 
const LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

// Basic types needed for the demo
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AccountIdentifier {
    pub hash: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Subaccount(pub [u8; 32]);

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Tokens {
    pub e8s: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Memo(pub u64);

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub memo: Memo,
    pub amount: Tokens,
    pub fee: Tokens,
    pub from_subaccount: Option<Subaccount>,
    pub to: AccountIdentifier,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AccountBalanceArgs {
    pub account: AccountIdentifier,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct NeuronId {
    pub id: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ClaimOrRefreshNeuronFromAccount {
    pub controller: Option<Principal>,
    pub memo: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ClaimOrRefreshNeuronFromAccountResponse {
    pub result: Option<ClaimOrRefreshResult>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ClaimOrRefreshResult {
    Error(GovernanceError),
    NeuronId(NeuronId),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GovernanceError {
    pub error_type: i32,
    pub error_message: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ManageNeuronRequest {
    pub id: Option<NeuronId>,
    pub command: Option<ManageNeuronCommand>,
    pub neuron_id_or_subaccount: Option<NeuronIdOrSubaccount>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ManageNeuronResponse {
    pub command: Option<ManageNeuronResponseCommand>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum NeuronIdOrSubaccount {
    Subaccount(Vec<u8>),
    NeuronId(NeuronId),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ManageNeuronCommand {
    Configure(Configure),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ManageNeuronResponseCommand {
    Error(GovernanceError),
    Configure(()),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Configure {
    pub operation: Option<ConfigureOperation>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ConfigureOperation {
    IncreaseDissolveDelay(IncreaseDissolveDelay),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IncreaseDissolveDelay {
    pub additional_dissolve_delay_seconds: u32,
}

// Application types
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum StakeNeuronError {
    TransferFailed(String),
    GovernanceCallFailed(String),
    ClaimFailed(String),
    ConfigureFailed(String),
    InvalidPrincipal(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StakeNeuronArgs {
    pub amount_e8s: u64,
    pub dissolve_delay_seconds: u32,
    pub memo: u64,
    pub neuron_controller: Option<Principal>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StakeNeuronResponse {
    pub neuron_id: NeuronId,
    pub transfer_block_height: u64,
}

const DEFAULT_SUBACCOUNT: Subaccount = Subaccount([0; 32]);

/// Stake ICP tokens to create a new neuron with the specified dissolve delay.
/// Note, this demo requires that the CANISTER has cycles.
///
/// If you want to stake a neuron from some other environment, your principal will need
/// to be accessible in that context.
#[update]
async fn stake_neuron(args: StakeNeuronArgs) -> Result<StakeNeuronResponse, StakeNeuronError> {
    let caller = ic_cdk::caller();
    let StakeNeuronArgs { amount_e8s, dissolve_delay_seconds, memo, neuron_controller } = args;
    let principal = neuron_controller.unwrap_or(caller);

    // We now check to see if there are funds available from the caller in a subaccount for the caller.
    todo!();
    // We also check ICRC-2, and then transfer any extra needed.  If the combined balances are not enough
    // return an error.
    todo!();

    // Check to see if the target account on governance is already funded (maybe this is a retry)
    let governance_account = compute_neuron_account(caller, memo);

    // check the balance...
    todo!();

    // We now transfer funds if needed.

    // We now have established we have sufficient funds.

    // Parse governance canister principal
    let governance_principal = Principal::from_text(GOVERNANCE_CANISTER_ID)
        .map_err(|e| StakeNeuronError::InvalidPrincipal(format!("Invalid governance canister ID: {}", e)))?;

    // Calculate the governance canister's account for neuron staking
    let governance_account = compute_neuron_account(caller, memo);
    
    // Step 1: Transfer ICP from this canister to the governance canister
    ic_cdk::println!(
        "Transferring {} e8s ICP from canister to governance canister for neuron creation",
        amount_e8s
    );

    // If the balance is not enough, we do this transfer.
    let transfer_args = TransferArgs {
        memo: Memo(memo),
        amount: Tokens { e8s: amount_e8s },
        fee: Tokens { e8s: 10_000 }, // Standard ICP transfer fee
        from_subaccount: None, // Use canister's default subaccount
        to: governance_account,
        created_at_time: None,
    };
    
    let ledger_principal = Principal::from_text(LEDGER_CANISTER_ID)
        .map_err(|e| StakeNeuronError::InvalidPrincipal(format!("Invalid ledger canister ID: {}", e)))?;
    
    let (transfer_result,): (Result<u64, String>,) = call(
        ledger_principal,
        "transfer",
        (transfer_args,),
    )
    .await
    .map_err(|e| StakeNeuronError::TransferFailed(format!("Transfer call failed: {:?}", e)))?;
    
    let transfer_block_height = transfer_result
        .map_err(|e| StakeNeuronError::TransferFailed(format!("Transfer failed: {}", e)))?;
    
    ic_cdk::println!("Transfer successful, block height: {}", transfer_block_height);



    // Step 2: Claim the neuron using the memo and controller
    let claim_request = ClaimOrRefreshNeuronFromAccount {
        controller: Some(caller),
        memo,
    };
    
    ic_cdk::println!("Claiming neuron with memo: {}", memo);
    
    let (claim_response,): (ClaimOrRefreshNeuronFromAccountResponse,) = call(
        governance_principal,
        "claim_or_refresh_neuron_from_account",
        (claim_request,),
    )
    .await
    .map_err(|e| StakeNeuronError::GovernanceCallFailed(format!("Claim call failed: {:?}", e)))?;
    
    // Extract neuron ID from the claim response
    let neuron_id = match claim_response.result {
        Some(ClaimOrRefreshResult::NeuronId(neuron_id)) => neuron_id,
        Some(ClaimOrRefreshResult::Error(err)) => {
            return Err(StakeNeuronError::ClaimFailed(format!("Claim failed: {:?}", err)));
        },
        None => {
            return Err(StakeNeuronError::ClaimFailed("No result in claim response".to_string()));
        }
    };
    
    ic_cdk::println!("Neuron claimed successfully, ID: {}", neuron_id.id);
    
    // Step 3: Configure the neuron with the requested dissolve delay
    if dissolve_delay_seconds > 0 {
        let configure_request = ManageNeuronRequest {
            id: Some(neuron_id.clone()),
            neuron_id_or_subaccount: Some(NeuronIdOrSubaccount::NeuronId(neuron_id.clone())),
            command: Some(ManageNeuronCommand::Configure(Configure {
                operation: Some(ConfigureOperation::IncreaseDissolveDelay(IncreaseDissolveDelay {
                    additional_dissolve_delay_seconds: dissolve_delay_seconds,
                })),
            })),
        };
        
        ic_cdk::println!(
            "Configuring neuron with dissolve delay: {} seconds ({} days)",
            dissolve_delay_seconds,
            dissolve_delay_seconds / 86400
        );
        
        let (configure_response,): (ManageNeuronResponse,) = call(
            governance_principal,
            "manage_neuron",
            (configure_request,),
        )
        .await
        .map_err(|e| StakeNeuronError::GovernanceCallFailed(format!("Configure call failed: {:?}", e)))?;
        
        // Check if configuration was successful
        match configure_response.command {
            Some(ManageNeuronResponseCommand::Configure(_)) => {
                ic_cdk::println!("Neuron configured successfully");
            },
            Some(ManageNeuronResponseCommand::Error(err)) => {
                return Err(StakeNeuronError::ConfigureFailed(format!("Configure failed: {:?}", err)));
            },
            None => {
                return Err(StakeNeuronError::ConfigureFailed("No command in configure response".to_string()));
            }
        }
    }
    
    Ok(StakeNeuronResponse {
        neuron_id,
        transfer_block_height,
    })
}

/// Get the current ICP balance of this canister
#[update]
async fn get_canister_balance() -> Result<Tokens, String> {
    let canister_id = ic_cdk::id();
    let account = AccountIdentifier {
        hash: compute_account_hash(&canister_id, &DEFAULT_SUBACCOUNT),
    };
    
    let balance_args = AccountBalanceArgs { account };
    
    let ledger_principal = Principal::from_text(LEDGER_CANISTER_ID)
        .map_err(|e| format!("Invalid ledger canister ID: {}", e))?;
    
    let (balance,): (Tokens,) = call(ledger_principal, "account_balance", (balance_args,))
        .await
        .map_err(|e| format!("Failed to get balance: {:?}", e))?;
    
    Ok(balance)
}

/// Get the account identifier where users should send ICP to fund this canister
#[query]
fn get_canister_account() -> Vec<u8> {
    let canister_id = ic_cdk::id();
    compute_account_hash(&canister_id, &DEFAULT_SUBACCOUNT)
}

/// Compute the neuron staking account for a given controller and memo
fn compute_neuron_account(controller: Principal, nonce: u64) -> AccountIdentifier {
    let governance_principal = Principal::from_text(GOVERNANCE_CANISTER_ID).unwrap();
    let subaccount = compute_neuron_staking_subaccount(controller, nonce);
    AccountIdentifier {
        hash: compute_account_hash(&governance_principal, &subaccount),
    }
}

/// Compute the subaccount for neuron staking
fn compute_neuron_staking_subaccount(controller: Principal, nonce: u64) -> Subaccount {
    let domain = b"neuron-stake";
    let domain_length: [u8; 1] = [domain.len() as u8];
    let mut hasher = Sha256::new();
    hasher.update(&domain_length);
    hasher.update(domain);
    hasher.update(controller.as_slice());
    hasher.update(&nonce.to_be_bytes());
    let hash = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash);
    Subaccount(result)
}

/// Compute account hash from principal and subaccount
fn compute_account_hash(principal: &Principal, subaccount: &Subaccount) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(&[10u8]); // account domain separator
    hasher.update(principal.as_slice());
    hasher.update(&subaccount.0);
    hasher.finalize().to_vec()
}

// Enable Candid export
ic_cdk::export_candid!();