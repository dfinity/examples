use bitcoin::Address;
use ic_btc_library::{
    get_current_fees_from_args, AddressType, BitcoinAgent, BitcoinCanister, BitcoinCanisterImpl,
    Fee, Network, Satoshi, TransferError, TransactionInfo,
};
use ic_cdk::{api::caller, export::Principal, print, trap};
use ic_cdk_macros::{query, update};
use std::{cell::RefCell, collections::HashMap, str::FromStr};

const MIN_CONFIRMATIONS: u32 = 0;
const NETWORK: Network = Network::Regtest;
const ADDRESS_TYPE: &AddressType = &AddressType::P2pkh;

thread_local! {
    // The Bitcoin wallet uses only a single Bitcoin agent to track all users' addresses.
    static BITCOIN_AGENT: RefCell<BitcoinAgent<BitcoinCanisterImpl>> =
        RefCell::new(BitcoinAgent::new(
            BitcoinCanisterImpl::new(NETWORK, ADDRESS_TYPE),
            ADDRESS_TYPE,
            MIN_CONFIRMATIONS
        ).unwrap());
}

/// Returns the user's `Principal`.
/// If the user isn't authenticated, then the anonymous principal is returned.
#[query]
fn whoami() -> Principal {
    caller()
}

// All other endpoints than `whoami` trap if the user isn't authenticated.
// TODO (ER-2527) Derive Bitcoin addresses for users (have to derive multiple addresses for a given principal)

/// Returns the user's `Address`.
async fn get_principal_address() -> Address {
    let caller_principal = caller();
    if caller_principal == Principal::anonymous() {
        trap("Caller principal wasn't obtained through Internet Identity.")
    }

    let derivation_path = caller_principal.as_slice();
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
    BITCOIN_AGENT
        .with(|bitcoin_agent| (*bitcoin_agent.borrow()).clone())
        .get_balance(principal_address, MIN_CONFIRMATIONS)
        .await
        .unwrap()
}

/// Returns 25th, 50th and 75th fees as percentiles in Satoshis/byte over the last 10,000 transactions.
#[update]
async fn get_fees() -> (Satoshi, Satoshi, Satoshi) {
    let get_current_fees_args =
        BITCOIN_AGENT.with(|bitcoin_agent| bitcoin_agent.borrow().get_current_fees_args());
    let current_fees = get_current_fees_from_args(get_current_fees_args)
        .await
        .unwrap();
    (current_fees[25], current_fees[50], current_fees[75])
}

/// Sends a transaction, transferring the specified Bitcoin amount to the provided address.
#[update]
async fn transfer(
    address: String,
    amount: Satoshi,
    fee: Satoshi,
    rbf: bool,
) -> Result<TransactionInfo, TransferError> {
    // pay attention not to use money of somebody else - don't create each time a BitcoinAgent otherwise we will always forget just spent UTXOs - before commiting, let say there is a single user
    // or could accept that they share UTXOs
    let principal_address = &get_principal_address().await;
    let payouts = &HashMap::from([(Address::from_str(&address).unwrap(), amount)]);

    let mut bitcoin_agent = BITCOIN_AGENT.with(|bitcoin_agent| bitcoin_agent.borrow().clone());
    bitcoin_agent
        .get_balance_update(principal_address)
        .await
        .unwrap();
    let transfer_result = bitcoin_agent
        .multi_transfer(payouts, principal_address, Fee::PerByte(fee), MIN_CONFIRMATIONS, rbf)
        .await;
    print(&format!(
        "{:?} {:?} {:?} {}",
        payouts,
        Fee::PerByte(fee),
        MIN_CONFIRMATIONS,
        rbf
    ));
    BITCOIN_AGENT.with(|global_bitcoin_agent| global_bitcoin_agent.replace(bitcoin_agent));
    transfer_result
}
