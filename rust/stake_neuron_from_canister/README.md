# Stake Neuron Example

This example demonstrates how to stake ICP tokens to create a neuron on the Network Nervous System (NNS) governance canister from within a Rust canister.

## Overview

This is a **demonstration canister** that shows the neuron staking process. In this example:

1. **The canister acts as a staking service** - Users call the canister to stake neurons on their behalf
2. **The canister manages its own ICP balance** - ICP must be transferred to the canister first
3. **Complete neuron lifecycle** - Transfer ICP → Claim neuron → Configure dissolve delay

## Important Note

This example is designed for **local testing and educational purposes**. The canister stakes neurons using its own ICP balance, which means:

- Users must first transfer ICP to the canister
- The canister becomes the controller of the created neurons
- For production use, you'd want more sophisticated account management

## Key Concepts

- **Neuron**: A stake of ICP tokens locked in the NNS governance canister that gives voting power
- **Dissolve Delay**: The time a neuron must wait before it can be dissolved and ICP withdrawn
- **Memo**: A unique identifier used to link ICP transfers to neuron creation
- **Voting Power**: Neurons with ≥6 months dissolve delay can vote on proposals and earn rewards

## Prerequisites

- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx) 0.25.0 or later
- [Rust](https://rustup.rs/) with wasm32 target
- `candid-extractor` tool: `cargo install candid-extractor`

## Running the Demo

The easiest way to see this in action is to run the complete demo script:

```bash
./demo.sh
```

This script will:
1. Start a local Internet Computer replica
2. Deploy a local ICP ledger with test tokens
3. Deploy the stake neuron canister  
4. Fund the canister with ICP
5. Demonstrate staking neurons with different dissolve delays

## Manual Testing

If you prefer to run the steps manually:

**Step 1: Start local replica**
```bash
dfx start --background --clean
```

**Step 2: Deploy the canister**
```bash
dfx deploy stake_neuron_backend
```

**Step 3: Fund the canister (you'll need a local ICP ledger for this)**
```bash
# Get the canister's account identifier
dfx canister call stake_neuron_backend get_canister_account

# Transfer ICP to the canister (requires local ledger setup)
# See demo.sh for complete setup
```

**Step 4: Stake neurons**
```bash
# Check canister balance
dfx canister call stake_neuron_backend get_canister_balance

# Stake 2 ICP with 1 year dissolve delay
dfx canister call stake_neuron_backend stake_neuron '(record {
  amount_e8s = 200_000_000 : nat64;
  dissolve_delay_seconds = 31_557_600 : nat32;
  memo = 12345 : nat64;
})'
```

## API Reference

### `stake_neuron(args: StakeNeuronArgs) -> Result<StakeNeuronResponse, StakeNeuronError>`

Stakes ICP tokens from the canister's balance to create a new neuron.

**Parameters:**
- `amount_e8s`: Amount to stake in e8s (1 ICP = 100,000,000 e8s)
- `dissolve_delay_seconds`: Initial dissolve delay in seconds
- `memo`: Unique identifier for this neuron (must be unique)

**Returns:**
- `neuron_id`: ID of the created neuron
- `transfer_block_height`: Block height of the ICP transfer

### `get_canister_balance() -> Result<Tokens, String>`

Returns the current ICP balance of the canister.

### `get_canister_account() -> AccountIdentifier`

Returns the account identifier where ICP should be sent to fund this canister.

### `get_staking_info() -> String`

Returns information about minimum staking requirements and common dissolve delays.

## Minimum Requirements

- **Amount**: 1 ICP (100,000,000 e8s)
- **Memo**: Must be unique for each neuron
- **Dissolve delay**: 0+ seconds (≥6 months needed for voting)

## Common Dissolve Delays

- **6 months**: 15,778,800 seconds (minimum for voting)
- **1 year**: 31,557,600 seconds
- **2 years**: 63,115,200 seconds  
- **8 years**: 252,460,800 seconds (maximum)

## Architecture

```
User → Canister → ICP Ledger → NNS Governance
                     ↓              ↓
               Transfer ICP    Claim & Configure
                              Neuron
```

1. **User calls canister** with staking parameters
2. **Canister transfers ICP** from its balance to governance canister's neuron subaccount
3. **Canister claims neuron** using the transfer memo and user's principal as controller
4. **Canister configures neuron** with the requested dissolve delay

## Example Usage

### Stake 5 ICP with 2-year dissolve delay

```bash
dfx canister call stake_neuron_backend stake_neuron '(record {
  amount_e8s = 500_000_000 : nat64;
  dissolve_delay_seconds = 63_115_200 : nat32;
  memo = 67890 : nat64;
})'
```

### Response Example

```
(
  variant {
    Ok = record {
      neuron_id = record { id = 123456789 : nat64 };
      transfer_block_height = 987654 : nat64;
    }
  }
)
```

## Implementation Details

The canister demonstrates the complete neuron staking flow by implementing:

1. **Subaccount computation** - Uses SHA256 hash of domain, controller, and nonce
2. **ICP transfer** - Sends ICP to the computed governance subaccount  
3. **Neuron claiming** - Calls `claim_or_refresh_neuron_from_account` on governance
4. **Neuron configuration** - Sets dissolve delay using `manage_neuron`

## Limitations

- **Single controller**: The calling user becomes the neuron controller
- **Canister balance**: Requires pre-funding the canister with ICP
- **Local testing only**: Uses hardcoded canister IDs for local deployment
- **No account management**: Simple balance model for demonstration

## Security Considerations

- **Memo uniqueness**: Each memo should be unique to avoid conflicts
- **Minimum stake**: Ensure sufficient ICP balance before staking
- **Principal identity**: The caller becomes the neuron controller
- **Balance checking**: Verify canister has sufficient ICP before operations

## Troubleshooting

**Error: Transfer failed**
- Check canister ICP balance with `get_canister_balance`
- Ensure sufficient funds for amount + transfer fee (10,000 e8s)

**Error: Claim failed**  
- Verify the memo is unique and hasn't been used before
- Wait a few seconds for the transfer to be processed

**Error: Configure failed**
- Check that the neuron was successfully created first
- Verify dissolve delay parameters are valid

## Learn More

- [NNS Governance Documentation](https://internetcomputer.org/docs/current/concepts/governance/nns/)
- [Neuron Management Guide](https://internetcomputer.org/docs/current/concepts/governance/neurons-voting/)
- [ICP Ledger Documentation](https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/)
- [Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)