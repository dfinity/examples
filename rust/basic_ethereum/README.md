# Basic Ethereum

This example demonstrates how to deploy a canister smart contract on the Internet Computer that can **send and receive Ether (ETH)** on the Ethereum network. The canister uses threshold ECDSA to sign Ethereum transactions and HTTPS outcalls to communicate with the Ethereum network via the EVM RPC canister.

## Architecture

This example internally leverages:
- [Threshold ECDSA](https://docs.internetcomputer.org/concepts/chain-key-cryptography/#chain-key-signatures-threshold-ecdsa-and-schnorr): Each user's Ethereum address is derived deterministically from the canister's master ECDSA key using a derivation path based on the user's IC principal. This means each user has a unique, stable Ethereum address controlled by the canister.
- [HTTPS outcalls](https://docs.internetcomputer.org/concepts/https-outcalls): The canister communicates with the Ethereum network via the [EVM RPC canister](https://github.com/dfinity/evm-rpc-canister) (canister ID `7hfb6-caaaa-aaaar-qadga-cai` on ICP mainnet), which forwards requests to public Ethereum RPC providers such as `https://ethereum-sepolia-rpc.publicnode.com`.

For a deeper understanding of the ICP ↔ ETH integration, see the [Ethereum integration](https://docs.internetcomputer.org/concepts/chain-fusion/ethereum).

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/basic_ethereum
```

### Deploy and test locally

The local icp-cli network supports real HTTPS outcalls, so `get_balance` and `transaction_count` work against live Ethereum Sepolia data without deploying to ICP mainnet.

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

`bash test.sh` verifies address derivation (threshold ECDSA), queries a known funded Sepolia address's balance via the raw EVM RPC canister interface, and queries its transaction count (nonce) via the high-level `evm_rpc_client` — demonstrating both usage patterns side by side.

`send_eth` requires a funded Ethereum address and is not covered in the automated tests. See [Sending ETH](#sending-eth) below.

### Deploy to ICP mainnet

This example defines two mainnet environments in `icp.yaml`:

| Environment | Ethereum network | ECDSA key | Use case |
|-------------|-----------------|-----------|----------|
| `local` | Sepolia testnet | `test_key_1` | Local development with real HTTPS outcalls |
| `staging` | Sepolia testnet | `test_key_1` | Testing on ICP mainnet with free Sepolia ETH |
| `production` | Ethereum mainnet | `key_1` | Production deployments with real ETH |

Both mainnet environments deploy only the backend canister and point it to the [shared EVM RPC canister](https://github.com/dfinity/evm-rpc-canister) (`7hfb6-caaaa-aaaar-qadga-cai`) already running on ICP mainnet.

**Deploy to staging (Sepolia):**

```bash
icp deploy -e staging
```

**Deploy to production (Ethereum mainnet):**

```bash
icp deploy -e production
```

## Interacting with the deployed canister

### Get your Ethereum address

Each IC principal gets a unique, stable Ethereum address controlled by this canister. The address is derived deterministically from the principal using the canister's threshold ECDSA key — the same principal always maps to the same address.

Passing `null` returns the address for your own IC principal (the identity you are calling with):

```bash
icp canister call backend ethereum_address '(null)'
# Returns your Ethereum address, e.g. ("0x378a452B20d1f06008C06c581b1656BdC5313c0C")
```

You can also look up the address for any other IC principal:

```bash
icp canister call backend ethereum_address '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")'
# Returns e.g. ("0x8d68f7B3cdb40A2E77071077658b01A9EA4B040F")
```

### Check a balance and transaction count

Query the ETH balance (in Wei) for any Ethereum address:

```bash
icp canister call backend get_balance '(opt "0x378a452B20d1f06008C06c581b1656BdC5313c0C")'
```

Query the transaction count for any Ethereum address using the high-level `EvmRpcClient`. This calls `eth_getTransactionCount`, which returns the **nonce** — the number of transactions sent *from* the address (outgoing only, not received):

```bash
icp canister call backend transaction_count_with_client '(opt "0x378a452B20d1f06008C06c581b1656BdC5313c0C", null)'
```

Passing `null` uses the derived Ethereum address of your calling IC principal:

```bash
icp canister call backend transaction_count_with_client '(null, null)'
```

### Sending ETH

To send ETH, your derived Ethereum address must be funded first:

1. Get your Ethereum address (see above) — this is the address managed by the canister for your IC principal.
2. Get some Sepolia ETH from [Alchemy's Sepolia faucet](https://www.alchemy.com/faucets/ethereum-sepolia).
3. Send Sepolia ETH to your address using any Ethereum wallet (e.g. MetaMask).
4. Once the transaction has at least one confirmation, verify the balance:

```bash
icp canister call backend get_balance '(null)'
```

Then send ETH (amount in Wei):

```bash
icp canister call backend send_eth '("0xdd2851Cdd40aE6536831558DD46db62fAc7A844d", 1)'
```

Returns the transaction hash. Track it on [Sepolia Etherscan](https://sepolia.etherscan.io/).

> **Note:** Due to the replicated nature of HTTPS outcalls, errors such as "transaction already known" or "nonce too low" may be reported even if the transaction was successfully broadcast. Verify by checking Etherscan or confirming that the transaction count for the address increased.

## RPC providers and API keys

The example uses [PublicNode](https://ethereum-sepolia-rpc.publicnode.com) by default — a free, no-registration provider that works out of the box locally and on mainnet. This is sufficient for getting started and automated testing.

For production deployments, the EVM RPC canister supports premium providers (Alchemy, Ankr, BlockPi) that offer higher rate limits and reliability when configured with API keys. To enable them locally:

1. Register as a provider admin on the locally deployed EVM RPC canister:
```bash
icp canister call evm_rpc authorize '(record { auth = variant { RegisterProvider }; principal = principal "YOUR_PRINCIPAL" })'
```

2. Update the desired provider with your API key:
```bash
icp canister call evm_rpc updateProvider '(record { providerId = 6; apiKey = opt "YOUR_ALCHEMY_API_KEY" })'
```

3. In `backend/state.rs`, change `evm_rpc_services()` to use `None` to include all configured providers (better consensus across multiple providers):
```rust
EthereumNetwork::Sepolia => RpcServices::EthSepolia(None),
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app. For this example the following aspects are particularly relevant:

- [Certify query responses if they are relevant for security](https://docs.internetcomputer.org/guides/security/data-integrity-and-authenticity/#certified-variables): since the app offers a method to read balances.
- [Use a governance framework like SNS to make a canister have a decentralized controller](https://docs.internetcomputer.org/guides/security/canister-control/#use-a-governance-framework-such-as-the-sns-to-control-your-canisters): decentralized control may be essential for canisters holding ETH on behalf of users.
