# Neuron Staking from CLI

This example demonstrates how to stake ICP to create an NNS Governance neuron from a Rust CLI using `ic-agent`. It shows the two-step process that the ledger and governance canisters expect.

Before diving in, you may want to try [nns.internetcomputer.org](https://nns.internetcomputer.org/) first — it offers an interactive introduction to staking neurons and voting without writing any code.

## What you can learn

### 1. Neuron Staking Subaccount Calculation

The neuron staking subaccount is the most critical thing to get right.
If ICP is sent to the wrong subaccount it is permanently lost — the Governance canister cannot retrieve it.

The subaccount is a SHA-256 hash over these four inputs in order:

1. `12` — the length of the domain separator (one byte)
2. `"neuron-stake"` — the domain separator
3. controller principal as raw bytes (not the text form)
4. nonce as big-endian u64

```rust
pub fn compute_neuron_staking_subaccount(controller: Principal, nonce: u64) -> Subaccount {
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

1. **Transfer ICP** to the Governance canister at the computed staking subaccount.
2. **Claim the neuron** by calling `claim_or_refresh_neuron_from_account` on Governance.

The `memo` in the ledger transfer is just a label on the transaction and does not need to match anything in step 2. What matters is that the `nonce` (used to derive the staking subaccount in step 1) matches the `memo` field passed to `claim_or_refresh_neuron_from_account` in step 2 — that is how Governance recomputes the expected subaccount and verifies the funds.

Note: **anyone can complete step 2**. The neuron's controller is determined entirely by the subaccount derived in step 1, regardless of who calls `claim_or_refresh_neuron_from_account`.

The minimum staking amount enforced by NNS Governance is 100,000,000 e8s (1 ICP). The ICP transfer fee of 10,000 e8s is charged on top.

> **A simpler alternative**: NNS Governance now supports a `create_neuron` method that accepts an ICRC-2 approval, handling the transfer atomically. This is safer because the ICP stays in the caller's account until Governance successfully claims it — there is no window where funds can be stranded at the staking subaccount. icp-cli support for this flow is being tracked at [dfinity/icp-cli#637](https://github.com/dfinity/icp-cli/pull/637).

## Build and run from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) v1.85+ with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/stake_neuron_from_cli
```

### Run locally

```bash
icp network start -d
bash test.sh
icp network stop
```

`test.sh` creates a test identity, funds it with ICP from the local NNS, builds the CLI binary, and runs the full staking flow.

### Run on mainnet

```bash
cargo build --release

./target/release/stake_neuron_from_cli \
  --identity /path/to/identity.pem \
  --url https://icp-api.io \
  --amount-e8s 100000000 \
  --nonce 0
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
