use candid::Principal;
use ic_cdk::api::msg_caller;

#[ic_cdk::query]
fn whoami() -> Principal {
    msg_caller()
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
