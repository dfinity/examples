# Neuron Staking from CLI

A Rust CLI application demonstrating how to stake ICP to create a neuron on the Internet Computer's Network Nervous System (NNS).

## What You Can Learn

This example teaches the key implementation details required for ICRC-1 neuron staking:

### 1. **Neuron Staking Subaccount Calculation**

Learn how to compute the deterministic subaccount used for neuron staking:

```rust
fn compute_neuron_staking_subaccount_bytes(controller: &Principal, nonce: u64) -> [u8; 32] {
    let domain_length: [u8; 1] = [b"neuron-stake".len() as u8];
    let mut hasher = Sha256::new();
    hasher.update(&domain_length);
    hasher.update(b"neuron-stake");
    hasher.update(controller.as_slice());
    hasher.update(&nonce.to_be_bytes());
    hasher.finalize().into()
}
```

**Implementation details:**
- Uses SHA256 hashing with domain separation (`"neuron-stake"`)
- Combines controller principal + nonce for uniqueness
- Creates deterministic subaccount from principal and nonce in the way NNS Governance does

### 2. **ICP Ledger Account Identifier Format**

Understand how to create the ICP Ledger's account identifier structure:

```rust
impl AccountIdentifier {
    fn new(principal: &Principal, subaccount: Option<[u8; 32]>) -> Self {
        let mut hasher = Sha224::new();  // Uses SHA-224, not SHA-256!
        hasher.update(b"\x0Aaccount-id");
        hasher.update(principal.as_slice());
        
        let sub_account = subaccount.unwrap_or([0u8; 32]);
        hasher.update(&sub_account);
        
        AccountIdentifier {
            hash: hasher.finalize().into(), // 28-byte SHA-224 hash
        }
    }
    
    // Serializes as 32-byte blob: 4-byte CRC32 checksum + 28-byte hash
    fn to_vec(&self) -> Vec<u8> {
        let checksum = crc32fast::hash(&self.hash);
        let checksum_bytes = checksum.to_be_bytes();
        [&checksum_bytes[..], &self.hash[..]].concat()
    }
}
```

**Implementation details:**
- Core identifier is 28-byte SHA-224 hash
- Wire format to ledger is a 32-byte blob (CRC32 checksum + SHA-224)
- Domain length plus domain id: `b"\x0Aaccount-id"`

### 3. **Two-Step Neuron Creation Process**

See the complete neuron staking flow:

1. **Transfer ICP** → Transfer tokens to governance canister's computed subaccount
2. **Claim Neuron** → Call `claim_or_refresh_neuron_from_account` to create the neuron

```rust
// Step 1: Transfer ICP to the computed subaccount
let to_account = AccountIdentifier::new(&governance_principal, Some(subaccount));
let transfer_result = ledger.transfer(TransferArgs {
    memo: nonce,
    amount: Tokens { e8s: amount },
    to: to_account.to_vec(),
    // ...
}).await?;

// Step 2: Claim the neuron using the same nonce
let claim_result = governance.claim_or_refresh_neuron_from_account(
    ClaimOrRefreshNeuronFromAccount {
        controller: Some(controller),
        memo: nonce,
    }
).await?;
```

### 4. **Identity Management with ic-agent**

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

## Local Testing with setup_and_run.sh

The `setup_and_run.sh` script provides a complete local testing environment:

```bash
#!/bin/bash
# 1. Start local IC replica
dfx start --clean --background

# 2. Deploy NNS canisters with canonical IDs
dfx deploy icp_ledger --specified-id ryjl3-tyaaa-aaaaa-aaaba-cai
dfx deploy nns_governance --specified-id rrkah-fqaaa-aaaaa-aaaaq-cai

# 3. Run the CLI tool
cargo run -- --identity "$HOME/.config/dfx/identity/default/identity.pem" \
              --url "http://127.0.0.1:4943" \
              --amount 100010000 \
              --nonce 42
```

**What this teaches:**
- How to set up local NNS environment
- Importance of canonical canister IDs for proper inter-canister calls
- Complete governance and ledger initialization arguments
- Integration between `dfx` and Rust tooling


## Running the Example

### Prerequisites

1. **Install dfx**: Follow [DFINITY SDK installation](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
2. **Rust toolchain**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh`

### Option 1: Manual CLI Usage

```bash
# Build the CLI
cargo build --release

# Run with your own identity 
./target/release/stake_neuron_from_cli \
    --identity ~/.config/dfx/identity/default/identity.pem \
    --url [YOUR LOCAL TEST ENVIRONMENT URL] \
    --amount 100010000 \
    --nonce 42
```

### Option 2: Local Testing Environment

```bash
# Set up local NNS and run example
chmod +x setup_and_run.sh
./setup_and_run.sh
```

This will:
1. Start a local IC replica
2. Deploy ICP Ledger and NNS Governance canisters
3. Run the staking example
4. Leave the environment running for inspection

### Inspecting Results

After running locally, you can verify the neuron was created:

```bash
# Query the governance canister
dfx canister call nns_governance list_neurons "(record {neuron_ids=vec{}; include_neurons_readable_by_caller=true})"
```
