---
keywords: [ advanced, rust, ethereum, eth, integration, ethereum integration ]
---

# Basic Ethereum

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/basic_ethereum)

## Overview

This tutorial will walk you through how to deploy a
sample [canister smart contract](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/overview)
**that can send and receive Ether (ETH)** on the Internet Computer.

## Architecture

This example internally leverages
the [threshold ECDSA](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/t-ecdsa)
and [HTTPs outcalls](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/https-outcalls/https-outcalls-overview)
features of the Internet Computer.

For a deeper understanding of the ICP < > ETH integration, see
the [Ethereum integration overview](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/overview).

## Prerequisites

* [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).

## Step 1: Building and deploying sample code

### Clone the smart contract

To clone and build the smart contract in **Rust**:

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/basic_ethereum
git submodule update --init --recursive
```

**If you are using MacOS, you'll need to install Homebrew and run `brew install llvm` to be able to compile the example.
**

### Acquire cycles to deploy

Deploying to the Internet Computer
requires [cycles](https://internetcomputer.org/docs/current/developer-docs/setup/cycles) (the equivalent of "gas" in
other blockchains). You can get free cycles from
the [cycles faucet](https://internetcomputer.org/docs/current/developer-docs/setup/cycles/cycles-faucet.md).

### Deploy the smart contract to the Internet Computer

```bash
dfx deploy --ic basic_ethereum --argument '(opt record {ethereum_network = opt variant {Sepolia}; ecdsa_key_name = opt variant {TestKey1}})'
```

#### What this does

- `dfx deploy` tells the command line interface to `deploy` the smart contract
- `--ic` tells the command line to deploy the smart contract to the mainnet ICP blockchain
- `--argument (opt record {ethereum_network = opt variant {Sepolia}; ecdsa_key_name = opt variant {TestKey1}})`
  initializes the smart contract with the provided arguments:
    - `ethereum_network = opt variant {Sepolia}`: the canister uses
      the [Ethereum Testnet Sepolia](https://github.com/ethereum-lists/chains/blob/master/_data/chains/eip155-11155111.json)
      network.
    - `ecdsa_key_name = opt variant {TestKey1}`: the canister uses a test key for signing via threshold ECDSA that is
      available on the ICP mainnet.
      See [signing messages](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/signing-messages#signing-messages-1)
      for more details.

If successful, you should see an output that looks like this:

```bash
Deploying: basic_ethereum
Building canisters...
...
Deployed canisters.
URLs:
Candid:
    basic_ethereum: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=<YOUR-CANISTER-ID>
```

Your canister is live and ready to use! You can interact with it using either the command line or using the Candid UI,
which is the link you see in the output above.

In the output above, to see the Candid Web UI for your ethereum canister, you would use the
URL `https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=<YOUR-CANISTER-ID>`. You should see the methods specified in the Candid file `basic_ethereum.did`.

## Step 2: Generating an Ethereum address

An Ethereum address can be derived from an ECDSA public key. To derive a user's specific address, identified on the IC
by a principal, the canister uses its own threshold ECDSA public key to derive a new public key deterministically for
each requested principal. To retrieve your Ethereum address, you can call the `ethereum_address` method on the
previously deployed canister:

```shell
dfx canister --ic call basic_ethereum ethereum_address
```

This will return an Ethereum address such as `("0x378a452B20d1f06008C06c581b1656BdC5313c0C")` that is tied to your
principal. Your address will be different. You can view such addresses on any Ethereum block explorer such as [Etherscan](https://etherscan.io/).

If you want to send some ETH to someone else, you can also use the above method to enquire about their Ethereum address
given their IC principal:

```shell
dfx canister --ic call basic_ethereum ethereum_address '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")'
```

This will return a different Ethereum address as the one above, such
as `("0x8d68f7B3cdb40A2E77071077658b01A9EA4B040F")`.

## Step 3: Receiving ETH

Now that you have your Ethereum address, let us send some (Sepolia) ETH to it:

1. Get some Sepolia ETH if you don't have any. You can for example use [this faucet](https://www.alchemy.com/faucets/ethereum-sepolia).
2. Send some Sepolia ETH to the address you obtained in the previous step. You can use any Ethereum wallet (e.g.,
   Metamask) to do so.

Once the transaction has at least one confirmation, which can take a few seconds,
you'll be able to see it in your Ethereum address's balance, which should be visible in an Ethereum block explorer,
e.g., https://sepolia.etherscan.io/address/0x378a452b20d1f06008c06c581b1656bdc5313c0c.

## Step 4: Sending ETH

You can send ETH using the `send_eth` endpoint on your canister, specifying an Ethereum destination address and an
amount in the smallest unit (Wei). For example, to send 1 Wei to `0xdd2851Cdd40aE6536831558DD46db62fAc7A844d`, run the following command:

```shell
dfx canister --ic call basic_ethereum send_eth '("0xdd2851Cdd40aE6536831558DD46db62fAc7A844d", 1)'
```

The `send_eth` endpoint sends ETH by executing the following steps:

1. Retrieving the transaction count for the sender's address at `Latest` block height. This is necessary because
   Ethereum transactions for a given sender's address are ordered by a `nonce`, which is a monotonically incrementally
   increasing non-negative counter.
2. Estimating the current transaction fees. For simplicity, the current gas fees are hard-coded with a generous limit. A
   real world application would dynamically fetch the latest transaction fees, for example using
   the [`eth_feeHistory`](https://github.com/internet-computer-protocol/evm-rpc-canister/blob/3cce151d4c1338d83e6741afa354ccf11dff41e8/candid/evm_rpc.did#L254)
   method in the [EVM-RPC canister](https://github.com/internet-computer-protocol/evm-rpc-canister/tree/main).
3. Building an Ethereum transaction ([EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)) to send the specified amount
   to the given receiver's address.
4. Signing the Ethereum transaction using
   the [sign_with_ecdsa API](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/signing-messages).
5. Sending the signed transaction to the Ethereum network using
   the [`eth_sendRawTransaction`](https://github.com/internet-computer-protocol/evm-rpc-canister/blob/3cce151d4c1338d83e6741afa354ccf11dff41e8/candid/evm_rpc.did#L261)
   method in the [EVM-RPC canister](https://github.com/internet-computer-protocol/evm-rpc-canister/tree/main).

The `send_eth` endpoint returns the hash of the transaction sent to the Ethereum network, which can for example be used
to track the transaction on an Ethereum blockchain explorer.

## Conclusion

In this tutorial, you were able to:

* Deploy a canister smart contract on the ICP blockchain that can receive and send ETH.
* Use a cycles faucet to deploy the canister to ICP blockchain on the mainnet for free.
* Connect the canister to the Ethereum Sepolia testnet.
* Send the canister some Sepolia ETH.
* Use the canister to send ETH to another Ethereum address.

Additional examples regarding the ICP < > ETH integration can be
found [here](https://internetcomputer.org/docs/current/developer-docs/multi-chain/examples#ethereum--evm-examples).

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to
the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the
Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security),
  since the app offers a method to read balances, for example.
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller),
  since decentralized control may be essential for canisters holding ETH on behalf of users.
