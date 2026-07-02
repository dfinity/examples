# Neuron Staking

This example demonstrates how to stake ICP to create an NNS Governance neuron from a canister. It shows the subaccount computation and two-step staking process required to correctly create a neuron.

## What you can learn

### 1. Neuron Staking Subaccount Calculation

The neuron staking subaccount is the most critical thing to get right when staking a neuron.
If you send your ICP to the wrong subaccount, the ICP will be permanently lost, as there will
not be a way to ask the Governance canister to retrieve it.

Therefore, test your implementation (like in this example project) against a known reference
before sending real ICP.

The subaccount is a domain-separated SHA-256 hash over the controller principal and nonce:

```rust
fn compute_neuron_staking_subaccount(controller: Principal, nonce: u64) -> Subaccount {
    let domain = b"neuron-stake";
    let mut hasher = Sha256::new();
    hasher.update([domain.len() as u8]);
    hasher.update(domain);
    hasher.update(controller.as_slice());
    hasher.update(nonce.to_be_bytes());
    Subaccount(hasher.finalize().into())
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

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

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

`icp.yaml` configures the local network with `nns: true`, which deploys the NNS canisters (ICP Ledger and Governance) at their mainnet IDs. `test.sh` funds the canister with ICP and exercises the full staking flow — both `compute_subaccount` and `stake_neuron` are tested locally.

### Testing on mainnet

To test against mainnet, deploy and fund the canister with ICP, then call `stake_neuron`:

```bash
icp deploy -e ic

# Fund the canister with ICP from your wallet, then call stake_neuron.
# amount: e8s to stake (minimum 100_000_000 = 1 ICP), nonce: unique u64 per neuron
icp canister call -e ic backend stake_neuron '(100_000_000 : nat64, 0 : nat64)'
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
