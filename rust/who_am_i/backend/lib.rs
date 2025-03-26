use candid::Principal;

#[ic_cdk::query]
fn whoami() -> Principal {
    ic_cdk::caller()
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
