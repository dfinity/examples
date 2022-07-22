use bitcoin::Address;
use ic_btc_library::{
    get_balance_from_args, get_current_fees_from_args, get_initialization_parameters_from_args,
    AddressType, BitcoinAgent, ManagementCanister, ManagementCanisterImpl, Network, Satoshi,
};
use ic_cdk::{api::caller, export::Principal, trap};
use ic_cdk_macros::{query, update};
use std::cell::RefCell;

const MIN_CONFIRMATIONS: u32 = 0;
const NETWORK: Network = Network::Regtest;
const ADDRESS_TYPE: &AddressType = &AddressType::P2pkh;

thread_local! {
    // The Bitcoin wallet uses only a single Bitcoin agent to track all users' addresses.
    static BITCOIN_AGENT: RefCell<BitcoinAgent<ManagementCanisterImpl>> =
        RefCell::new(BitcoinAgent::new(
            ManagementCanisterImpl::new(NETWORK),
            ADDRESS_TYPE,
            MIN_CONFIRMATIONS
        ).unwrap());
}

/// Initializes the Bitcoin agent.
/// This custom endpoint needs to be called once. This endpoint can then be removed in a canister upgrade.
#[update]
async fn initialize() {
    let get_initialization_parameters_args = BITCOIN_AGENT
        .with(|bitcoin_agent| bitcoin_agent.borrow().get_initialization_parameters_args());
    let initialization_parameters =
        get_initialization_parameters_from_args(get_initialization_parameters_args)
            .await
            .unwrap();
    BITCOIN_AGENT.with(|bitcoin_agent| {
        bitcoin_agent
            .borrow_mut()
            .initialize(initialization_parameters)
    });
}

/// Returns the user's `Principal`.
/// If the user isn't authenticated, then the anonymous principal is returned.
#[query]
fn whoami() -> Principal {
    caller()
}

// All other endpoints than `whoami` trap if the user isn't authenticated.
// TODO (ER-2527) Derive Bitcoin addresses for users (have to derive multiple addresses for a given principal)

/// Returns the user's `Principal` or traps if the user isn't authenticated.
fn get_authenticated_principal() -> Principal {
    let caller_principal = caller();
    if caller_principal == Principal::anonymous() {
        trap("Caller principal wasn't obtained through Internet Identity.")
    }
    caller_principal
}

/// Returns the user's `Address`.
async fn get_principal_address() -> Address {
    let caller_principal = get_authenticated_principal();
    let derivation_path = &[caller_principal.as_slice().to_vec()];
    BITCOIN_AGENT.with(|bitcoin_agent| {
        bitcoin_agent
            .borrow_mut()
            .add_address(derivation_path)
            .unwrap()
    })
}

/// Returns the user's address as a `String`.
#[update]
async fn get_principal_address_str() -> String {
    get_principal_address().await.to_string()
}

/// Returns the user's balance in `Satoshi`s.
#[update]
async fn get_balance() -> Satoshi {
    let principal_address = &get_principal_address().await;
    let get_utxos_args = BITCOIN_AGENT.with(|bitcoin_agent| {
        bitcoin_agent
            .borrow()
            .get_utxos_args(principal_address, MIN_CONFIRMATIONS)
    });
    get_balance_from_args(get_utxos_args).await.unwrap()
}

/// Returns the 25th, 50th, and 75th fee percentiles in Millisatoshi/byte over the last 10,000 transactions.
#[update]
async fn get_fees() -> (Satoshi, Satoshi, Satoshi) {
    get_authenticated_principal();
    let get_current_fees_args =
        BITCOIN_AGENT.with(|bitcoin_agent| bitcoin_agent.borrow().get_current_fees_args());
    let current_fees = get_current_fees_from_args(get_current_fees_args)
        .await
        .unwrap();
    (current_fees[25], current_fees[50], current_fees[75])
}
