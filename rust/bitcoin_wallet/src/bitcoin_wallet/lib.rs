use bitcoin::{Address, Network};
use ic_btc_library::{AddressType, BitcoinAgent, BitcoinCanister, BitcoinCanisterImpl};
use ic_btc_types::Satoshi;
use ic_cdk::{api::caller, export::Principal, trap};
use ic_cdk_macros::{query, update};
use std::cell::RefCell;

thread_local! {
    static BITCOIN_AGENT: RefCell<BitcoinAgent<BitcoinCanisterImpl>> =
        RefCell::new(BitcoinAgent::new(
            BitcoinCanisterImpl::new(),
            &Network::Regtest,
            &AddressType::P2pkh
        ));
}

#[query]
fn whoami() -> Principal {
    caller()
}

async fn get_principal_address() -> Address {
    let caller_principal = caller();
    if caller_principal == Principal::anonymous() {
        trap("Caller principal wasn't obtain through Internet Identity.")
    }

    let derivation_path = vec![caller_principal.as_slice().to_vec()];
    BITCOIN_AGENT.with(|bitcoin_agent| {
        bitcoin_agent
            .borrow_mut()
            .add_address(derivation_path, &AddressType::P2pkh)
            .unwrap()
    })
}

#[update]
async fn get_principal_address_str() -> String {
    get_principal_address().await.to_string()
}

#[update]
async fn get_balance() -> Satoshi {
    let principal_address = &get_principal_address().await;
    BITCOIN_AGENT
        .with(|bitcoin_agent| (*bitcoin_agent.borrow()).clone())
        .get_balance(principal_address, 0)
        .await
        .unwrap()
}
