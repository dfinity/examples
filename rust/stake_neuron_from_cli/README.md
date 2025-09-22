# Neuron Staking from CLI

This example demonstrates how to stake ICP to create an NNS Governance neuron.

## What you can learn

### 1. **Neuron Staking Subaccount Calculation**

The neuron staking subaccount is the most critical thing to get right when staking a neuron.  
If you send your ICP to the wrong subaccount, the ICP will be permanently lost, as there will not be a way
to ask the Governance canister to retrieve it.

Therefore, test your implementation (like in this example project) against a test version of Governance to ensure
that it is able to send the ICP to the right destination.

```rust
/// Compute the subaccount for neuron staking
fn compute_neuron_staking_subaccount(controller: Principal, nonce: u64) -> [u8; 32] {
    let domain_length: [u8; 1] = [b"neuron-stake".len() as u8];
    let mut hasher = Sha256::new();
    hasher.update(&domain_length);
    hasher.update(b"neuron-stake");
    hasher.update(controller.as_slice());
    hasher.update(&nonce.to_be_bytes());
    hasher.finalize().into()
}
```

### 2. **Two-Step Neuron Creation Process**

The neuron creation flow requires first making a ledger transfer, and then sending a message to tell
NNS Governance about the transfer you made so that it can create a neuron.

1. **Transfer ICP** → Transfer tokens to NNS Governance at the computed subaccount
2. **Claim Neuron** → Call `claim_or_refresh_neuron_from_account` to create the neuron

```rust
// Step 1: Transfer ICP to the computed subaccount
let to_account = AccountIdentifier::new( & governance_principal, & subaccount);
let transfer_args = TransferArgs {
memo: Memo(args.nonce),
amount: Tokens::from_e8s(args.amount),
fee: DEFAULT_FEE, // Standard ICP transfer fee
from_subaccount: None,
to: to_account,
created_at_time: None,
};

let transfer_result_bytes = agent
.update( & Principal::from_text(ICP_LEDGER_CANISTER_ID) ?, "transfer")
.with_arg(Encode!(&transfer_args)?)
.call_and_wait()
.await?;

// Step 2: Claim the neuron using the same nonce
let claim_request = ClaimOrRefreshNeuronFromAccount {
controller: Some(controller),
memo: args.nonce,
};

let claim_result_bytes = agent
.update(
& governance_principal,
"claim_or_refresh_neuron_from_account",
)
.with_arg(Encode!(&claim_request)?)
.call_and_wait()
.await?;
```

### 3. **Identity Management with ic-agent**

Additionally, you can see how you can use ic-agent to use different key formats to send messages.

```rust
async fn load_identity(identity_path: &PathBuf) -> Result<Box<dyn Identity>, Box<dyn std::error::Error>> {
    let pem_content = std::fs::read_to_string(identity_path)?;

    // Try different identity formats
    if let Ok(identity) = BasicIdentity::from_pem(pem_content.as_bytes()) {
        return Ok(Box::new(identity));
    }

    if let Ok(identity) = Secp256k1Identity::from_pem(pem_content.as_bytes()) {
        return Ok(Box::new(identity));
    }

    Err("Could not parse identity file".into())
}
```

## Running the Example

### Prerequisites

1. **Install dfx**:
   Follow [DFINITY SDK installation](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
2. **Rust toolchain**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh`

### Run the script 
```bash
# Set up local NNS and run example
chmod +x setup_and_run.sh
./setup_and_run.sh
```

This will:

1. Start a local IC replica
2. Deploy ICP Ledger and NNS Governance canisters with basic configuration
3. Run the staking example
4. Leave the environment running for inspection

### Inspecting Results

After running locally, you can verify the neuron was created:

```bash
# Query the governance canister
dfx canister call nns_governance list_neurons "(record {neuron_ids=vec{}; include_neurons_readable_by_caller=true})"
```
