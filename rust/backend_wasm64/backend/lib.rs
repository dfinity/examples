#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
