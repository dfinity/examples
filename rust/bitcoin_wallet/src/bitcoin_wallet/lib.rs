use bitcoin::Address;
use ic_btc_library::{AddressType, BitcoinAgent, BitcoinCanister, BitcoinCanisterImpl, Satoshi, Network};
use ic_cdk::{api::caller, export::Principal, trap};
use ic_cdk_macros::{query, update};
use std::cell::RefCell;

thread_local! {
    // The Bitcoin wallet uses only a single Bitcoin agent to track all users' addresses.
    static BITCOIN_AGENT: RefCell<BitcoinAgent<BitcoinCanisterImpl>> =
        RefCell::new(BitcoinAgent::new(
            BitcoinCanisterImpl::new(Network::Regtest),
            &AddressType::P2pkh,
			0
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
        .get_balance(principal_address, 0)
        .await
        .unwrap()
}
