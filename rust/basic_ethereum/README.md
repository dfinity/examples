# Basic Ethereum

This example demonstrates how to deploy a canister smart contract on the Internet Computer that can **send and receive Ether (ETH)** on the Ethereum network. The canister uses threshold ECDSA to sign Ethereum transactions and HTTPS outcalls to communicate with the Ethereum network via the EVM RPC canister.

## Architecture

This example internally leverages:
- [Threshold ECDSA](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/t-ecdsa): Each user's Ethereum address is derived deterministically from the canister's master ECDSA key using a derivation path based on the user's IC principal. This means each user has a unique, stable Ethereum address controlled by the canister.
- [HTTPS outcalls](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/https-outcalls/https-outcalls-overview): The canister communicates with the Ethereum network via the [EVM RPC canister](https://github.com/internet-computer-protocol/evm-rpc-canister/tree/main) (canister ID `7hfb6-caaaa-aaaar-qadga-cai` on ICP mainnet).

For a deeper understanding of the ICP ↔ ETH integration, see the [Ethereum integration overview](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/overview).

> **Note:** `get_balance`, `transaction_count`, and `send_eth` require live HTTPS outcalls to the Ethereum network and work only when deployed on ICP mainnet. The `ethereum_address` function can be tested locally as it only uses threshold ECDSA, which is available in the local replica.

## Build and deploy from the command line

### Prerequisites
- [Node.js](https://nodejs.org/)
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/basic_ethereum
```

### Deploy and test locally

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

### Deploy to ICP mainnet (Sepolia testnet)

To interact with the live Ethereum Sepolia testnet:

```bash
icp deploy --network ic --argument '(opt record {ethereum_network = opt variant {Sepolia}; ecdsa_key_name = opt variant {TestKey1}})'
```

- `ethereum_network = opt variant {Sepolia}`: uses the [Ethereum Testnet Sepolia](https://github.com/ethereum-lists/chains/blob/master/_data/chains/eip155-11155111.json).
- `ecdsa_key_name = opt variant {TestKey1}`: uses a test key for threshold ECDSA signing available on ICP mainnet. See [signing messages](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/t-ecdsa) for more details.

For production use with real ETH on Ethereum mainnet:

```bash
icp deploy --network ic --argument '(opt record {ethereum_network = opt variant {Mainnet}; ecdsa_key_name = opt variant {ProductionKey1}})'
```

## Interacting with the deployed canister

### Step 1: Get your Ethereum address

An Ethereum address is derived from the caller's IC principal via the canister's threshold ECDSA key. The same principal always maps to the same Ethereum address:

```bash
icp canister call backend ethereum_address '(null)'
# Returns e.g. ("0x378a452B20d1f06008C06c581b1656BdC5313c0C")
```

To get the Ethereum address for a different principal:

```bash
icp canister call backend ethereum_address '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")'
# Returns e.g. ("0x8d68f7B3cdb40A2E77071077658b01A9EA4B040F")
```

You can view these addresses on any Ethereum block explorer such as [Etherscan](https://etherscan.io/).

### Step 2: Receive ETH (mainnet only)

1. Get some Sepolia ETH from a faucet, for example [Alchemy's Sepolia faucet](https://www.alchemy.com/faucets/ethereum-sepolia).
2. Send Sepolia ETH to the address from Step 1 using any Ethereum wallet (e.g. MetaMask).
3. Once the transaction has at least one confirmation, verify the balance on [Sepolia Etherscan](https://sepolia.etherscan.io/).

### Step 3: Check your balance (mainnet only)

```bash
icp canister call backend get_balance '(null)'
```

### Step 4: Send ETH (mainnet only)

Send 1 Wei to a destination address:

```bash
icp canister call backend send_eth '("0xdd2851Cdd40aE6536831558DD46db62fAc7A844d", 1)'
```

The `send_eth` endpoint executes the following steps:

1. Retrieves the transaction count (nonce) for the sender's address at `Latest` block height. Ethereum transactions are ordered by nonce, a monotonically increasing counter.
2. Estimates transaction fees. For simplicity, fees are hard-coded with a generous limit. A production application would dynamically fetch the latest fees via the [`eth_feeHistory`](https://github.com/internet-computer-protocol/evm-rpc-canister/blob/3cce151d4c1338d83e6741afa354ccf11dff41e8/candid/evm_rpc.did#L254) method.
3. Builds an [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559) transaction.
4. Signs the transaction using the [sign_with_ecdsa API](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/t-ecdsa).
5. Sends the signed transaction to the Ethereum network via [`eth_sendRawTransaction`](https://github.com/internet-computer-protocol/evm-rpc-canister/blob/3cce151d4c1338d83e6741afa354ccf11dff41e8/candid/evm_rpc.did#L261) in the EVM RPC canister.

Returns the transaction hash, which can be used to track the transaction on a block explorer.

> **Note:** Due to the replicated nature of HTTPS outcalls, errors such as "transaction already known" or "nonce too low" may be reported even if the transaction was successfully broadcast. Verify by checking Etherscan or confirming that the transaction count for the address increased.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

- [Certify query responses if they are relevant for security](https://docs.internetcomputer.org/guides/security/overview), since the app offers a method to read balances.
- [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://docs.internetcomputer.org/guides/security/overview), since decentralized control may be essential for canisters holding ETH on behalf of users.

Additional examples for the ICP ↔ ETH integration can be found in the [ICP developer docs](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/overview).
