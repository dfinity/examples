# Basic Ethereum

This example demonstrates how to deploy a canister smart contract on the Internet Computer that can **send and receive Ether (ETH)** on the Ethereum network. The canister uses threshold ECDSA to sign Ethereum transactions and HTTPS outcalls to communicate with the Ethereum network via the EVM RPC canister.

## Architecture

This example internally leverages:
- [Threshold ECDSA](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/t-ecdsa): Each user's Ethereum address is derived deterministically from the canister's master ECDSA key using a derivation path based on the user's IC principal. This means each user has a unique, stable Ethereum address controlled by the canister.
- [HTTPS outcalls](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/https-outcalls/https-outcalls-overview): The canister communicates with the Ethereum network via the [EVM RPC canister](https://github.com/dfinity/evm-rpc-canister) (canister ID `7hfb6-caaaa-aaaar-qadga-cai` on ICP mainnet), which forwards requests to public Ethereum RPC providers such as `https://ethereum-sepolia-rpc.publicnode.com`.

For a deeper understanding of the ICP ↔ ETH integration, see the [Ethereum integration overview](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/overview).

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

`bash test.sh` verifies address derivation (threshold ECDSA), queries the canister's Sepolia balance via the raw EVM RPC canister interface, and queries the transaction count via the high-level `evm_rpc_client` — demonstrating both usage patterns.

`send_eth` requires a funded canister wallet and is not covered in the automated tests. See [Sending ETH](#sending-eth) below.

### Deploy to ICP mainnet (Sepolia testnet)

```bash
icp deploy --network ic --argument '(opt record {ethereum_network = opt variant {Sepolia}; ecdsa_key_name = opt variant {TestKey1}})'
```

- `ethereum_network = opt variant {Sepolia}`: uses [Ethereum Testnet Sepolia](https://github.com/ethereum-lists/chains/blob/master/_data/chains/eip155-11155111.json).
- `ecdsa_key_name = opt variant {TestKey1}`: uses a test threshold ECDSA key available on ICP mainnet.

For production use with real ETH on Ethereum mainnet:

```bash
icp deploy --network ic --argument '(opt record {ethereum_network = opt variant {Mainnet}; ecdsa_key_name = opt variant {ProductionKey1}})'
```

## Interacting with the deployed canister

### Get your Ethereum address

An Ethereum address is derived from the caller's IC principal via the canister's threshold ECDSA key. The same principal always maps to the same Ethereum address:

```bash
icp canister call backend ethereum_address '(null)'
# Returns e.g. ("0x378a452B20d1f06008C06c581b1656BdC5313c0C")
```

To get the Ethereum address for a specific principal:

```bash
icp canister call backend ethereum_address '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")'
# Returns e.g. ("0x8d68f7B3cdb40A2E77071077658b01A9EA4B040F")
```

### Check a balance

Query the ETH balance (in Wei) for any Ethereum address:

```bash
icp canister call backend get_balance '(opt "0x378a452B20d1f06008C06c581b1656BdC5313c0C")'
```

### Sending ETH

To send ETH the canister's wallet must be funded:

1. Get your canister's Ethereum address (see above).
2. Get some Sepolia ETH from [Alchemy's Sepolia faucet](https://www.alchemy.com/faucets/ethereum-sepolia).
3. Send Sepolia ETH to the canister's address using any Ethereum wallet (e.g. MetaMask).
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

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app. For this example the following aspects are particularly relevant:

- [Certify query responses if they are relevant for security](https://docs.internetcomputer.org/guides/security/data-integrity-and-authenticity/#certified-variables): since the app offers a method to read balances.
- [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://docs.internetcomputer.org/guides/security/overview): decentralized control may be essential for canisters holding ETH on behalf of users.
