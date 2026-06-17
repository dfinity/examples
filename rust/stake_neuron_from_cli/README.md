# Neuron Staking

This example demonstrates how to stake ICP to create an NNS Governance neuron from a canister. It shows the subaccount computation and two-step staking process required to correctly create a neuron.

> **Mainnet required for full staking**: The `stake_neuron` endpoint makes inter-canister calls to the ICP Ledger (`ryjl3-tyaaa-aaaaa-aaaba-cai`) and NNS Governance (`rrkah-fqaaa-aaaaa-aaaaq-cai`) canisters, which only exist on mainnet. The `compute_subaccount` query can be tested locally.

## What you can learn

### 1. Neuron Staking Subaccount Calculation

The neuron staking subaccount is the most critical thing to get right when staking a neuron.
If you send your ICP to the wrong subaccount, the ICP will be permanently lost, as there will
not be a way to ask the Governance canister to retrieve it.

Therefore, test your implementation (like in this example project) against a known reference
before sending real ICP.

The subaccount is a domain-separated SHA-256 hash over the controller principal and nonce:

```rust
fn compute_neuron_staking_subaccount(controller: Principal, nonce: u64) -> [u8; 32] {
    let domain = b"neuron-stake";
    let domain_length: [u8; 1] = [domain.len() as u8];
    let mut hasher = Sha256::new();
    hasher.update(domain_length);
    hasher.update(domain);
    hasher.update(controller.as_slice());
    hasher.update(nonce.to_be_bytes());
    hasher.finalize().into()
}
```

### 2. Two-Step Neuron Creation Process

The neuron creation flow requires first making a ledger transfer, and then sending a message to tell
NNS Governance about the transfer you made so that it can create a neuron.

1. **Transfer ICP** — Transfer tokens to NNS Governance at the computed subaccount
2. **Claim Neuron** — Call `claim_or_refresh_neuron_from_account` to register the transfer as a neuron

The nonce used in step 1 (as the transfer memo) must match the nonce used in step 2.
The minimum staking amount enforced by NNS Governance is 1 ICP (100,000,000 e8s).
The standard ICP transfer fee of 0.0001 ICP (10,000 e8s) is deducted on top of the staked amount.

```rust
// Step 1: Transfer ICP to the computed subaccount
let transfer_args = TransferArgs {
    memo: Memo(nonce),
    amount: Tokens { e8s: amount },
    fee: Tokens { e8s: 10_000 },
    to: AccountIdentifier { hash: build_account_identifier(governance_principal, &subaccount) },
    ..
};

// Step 2: Claim the neuron using the same nonce
let claim_request = ClaimOrRefreshNeuronFromAccount {
    controller: Some(canister_id),
    memo: nonce,
};
```

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/stake_neuron_from_cli
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

The `bash test.sh` script verifies the `compute_subaccount` query locally.

### Testing stake_neuron on mainnet

To test the full staking flow you need a canister deployed to mainnet that holds ICP:

```bash
# Deploy to mainnet
icp deploy --network ic

# Fund the canister with ICP (from your wallet), then call stake_neuron.
# amount: e8s to stake (minimum 100_000_000 = 1 ICP), nonce: unique u64 per neuron
icp canister call --network ic backend stake_neuron '(100010000 : nat64, 0 : nat64)'
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
