---
keywords: [advanced, motoko, bitcoin, bitcoin integration, btc]
---

# Basic Bitcoin

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/basic_bitcoin)

## Overview 
This tutorial will walk you through how to deploy a sample [canister smart contract](https://wiki.internetcomputer.org/wiki/Canister_smart_contract) **that can send and receive Bitcoin** on the Internet Computer.

## Architecture

This example internally leverages the [ECDSA
API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key),
[Schnorr API](https://internetcomputer.org/docs/current/references/ic-interface-spec#ic-sign_with_schnorr), and [Bitcoin
API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin-api)
of the Internet Computer.

For a deeper understanding of the ICP < > BTC integration, see the [Bitcoin integration documentation](/docs/current/developer-docs/multi-chain/bitcoin/overview).

## Prerequisites

* [x] Install the [IC
  SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx). For local testing, `dfx >= 0.22.0` is required.

:::info 
This example is designed to be deployed on the mainnet. It will return errors when deployed locally; these errors are expected.
:::

## Step 1: Building and deploying sample code

### Clone the smart contract

To clone and build the smart contract in **Motoko**:

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/basic_bitcoin
```

### Install MOPS

[Install](https://docs.mops.one/quick-start#2-install-mops-cli) the MOPS package
manager, e.g., by running

```bash
curl -fsSL cli.mops.one/install.sh | sh
```

### Acquire cycles to deploy

Deploying to the Internet Computer requires [cycles](https://internetcomputer.org/docs/current/developer-docs/setup/cycles) (the equivalent of "gas" in other blockchains). You can get free cycles from the [cycles faucet](https://internetcomputer.org/docs/current/developer-docs/setup/cycles/cycles-faucet.md).

### Deploy the smart contract to the Internet Computer

```bash
dfx deploy --network=ic basic_bitcoin --argument '(variant { testnet })'
```

#### What this does
- `dfx deploy` tells the command line interface to `deploy` the smart contract
- `--network=ic` tells the command line to deploy the smart contract to the mainnet ICP blockchain
- `--argument '(variant { Testnet })'` passes the argument `Testnet` to initialize the smart contract, telling it to connect to the Bitcoin testnet

**We're initializing the canister with `variant { testnet }` so that the
canister connects to the [Bitcoin testnet](https://en.bitcoin.it/wiki/Testnet).
To be specific, this connects to `Testnet3`, which is the current Bitcoin test
network used by the Bitcoin community. This means that the addresses generated
in the smart contract can only be used to receive or send funds only on Bitcoin
testnet.**


If successful, you should see an output that looks like this:

```bash
Deploying: basic_bitcoin
Building canisters...
...
Deployed canisters.
URLs:
Candid:
    basic_bitcoin: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=<YOUR-CANISTER-ID>
```

Your canister is live and ready to use! You can interact with it using either the command line or the Candid UI, which is the link you see in the output above.

In the output above, to see the Candid Web UI for your bitcoin canister, you would use the URL `https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=<YOUR-CANISTER-ID>`. Here are the two methods you will see:

* `public_key`
* `sign`

## Step 2: Generating a Bitcoin address

Bitcoin has different types of addresses (e.g. P2PKH, P2SH, P2TR). You may want
to check [this
article](https://bitcoinmagazine.com/technical/bitcoin-address-types-compared-p2pkh-p2sh-p2wpkh-and-more)
if you are interested in a high-level comparison of different address types.
These addresses can be generated from an ECDSA public key or a Schnorr
([BIP340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki))
public key. The example code showcases how your canister can generate and spend
from two types of addresses:
1. A [P2PKH address](https://en.bitcoin.it/wiki/Transaction#Pay-to-PubkeyHash)
   using the
   [ecdsa_public_key](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-method-ecdsa_public_key)
   API.
2. A [P2TR
   address](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)
   where the funds can be spent using the raw (untweaked) internal key
   (so-called P2TR key path spend, but untweaked). The advantage of this
   approach compared to P2TR script spends is its significantly smaller fee per
   transaction because checking the transaction signature is analogous to P2PK
   but uses Schnorr instead of ECDSA. IMPORTANT: Note that
   [BIP341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki#cite_note-23)
   advises against using taproot addresses that can be spent with an untweaked
   key. This precaution is to prevent attacks that can occur when creating
   taproot multisigner addresses using specific multisignature schemes. However,
   the Schnorr API of the internet computer does not support Schnorr
   multisignatures.

Note that P2TR *key path* spending with a tweaked key is currently not available
on the IC because the threshold Schnorr signing interface does not allow
applying BIP341 tweaks to the private key. In contrast, the
tweaked public key is used to spend in the script path, which is availble on the
IC. For a technical comparison of different ways of how single-signer P2TR
addresses can be constructed and used, you may want to take a look at [this
post](https://bitcoin.stackexchange.com/a/111100) by Pieter Wuille.

On the Candid UI of your canister, click the "Call" button under
`get_${type}_address` to generate a `${type}` Bitcoin address, where `${type}`
is one of `[p2pkh, p2tr_raw_key_spend]`.

Or, if you prefer the command line:

```bash
dfx canister --network=ic call basic_bitcoin get_${type}_address
```

* The Bitcoin address you see will be different from the one above because the
  ECDSA public key your canister retrieves is unique.

* We are generating a Bitcoin testnet address, which can only be
used for sending/receiving Bitcoin on the Bitcoin testnet.


## Step 3: Receiving bitcoin

Now that the canister is deployed and you have a Bitcoin address, it's time to receive
some testnet bitcoin. You can use one of the Bitcoin faucets, such as [coinfaucet.eu](https://coinfaucet.eu),
to receive some bitcoin.

Enter your address and click on "Send testnet bitcoins". In the example below we will use Bitcoin address `n31eU1K11m1r58aJMgTyxGonu7wSMoUYe7`, but you will use your address. The Bitcoin address you see will be different from the one above
because the ECDSA/Schnorr public key your canister retrieves is unique.


Once the transaction has at least one confirmation, which can take a few minutes,
you'll be able to see it in your canister's balance.

## Step 4: Checking your bitcoin balance

You can check a Bitcoin address's balance by using the `get_balance` endpoint on your canister.

In the Candid UI, paste in your canister's address, and click on "Call".

Alternatively, make the call using the command line. Be sure to replace `mheyfRsAQ1XrjtzjfU1cCH2B6G1KmNarNL` with your own generated address:

```bash
dfx canister --network=ic call basic_bitcoin get_balance '("mheyfRsAQ1XrjtzjfU1cCH2B6G1KmNarNL")'
```

Checking the balance of a Bitcoin address relies on the [bitcoin_get_balance](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance) API.

## Step 5: Sending bitcoin

You can send bitcoin using the `send_from_${type}` endpoint on your canister, where
`${type}` is on of `[p2pkh, p2tr_raw_key_spend]`.

In the Candid UI, add a destination address and an amount to send. In the example
below, we're sending 4'321 Satoshi (0.00004321 BTC) back to the testnet faucet.

Via the command line, the same call would look like this:

```bash
dfx canister --network=ic call basic_bitcoin send_from_p2pkh '(record { destination_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt"; amount_in_satoshi = 4321; })'
```

The `send_from_${type}` endpoint can send bitcoin by:

1. Getting the percentiles of the most recent fees on the Bitcoin network using the [bitcoin_get_current_fee_percentiles API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-method-bitcoin_get_current_fee_percentiles).
2. Fetching your unspent transaction outputs (UTXOs), using the [bitcoin_get_utxos API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-method-bitcoin_get_utxos).
3. Building a transaction, using some of the UTXOs from step 2 as input and the destination address and amount to send as output.
   The fee percentiles obtained from step 1 are used to set an appropriate fee.
4. Signing the inputs of the transaction using the
   [sign_with_ecdsa
   API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-method-sign_with_ecdsa)/\
   [sign_with_schnorr](https://org5p-7iaaa-aaaak-qckna-cai.icp0.io/docs#ic-sign_with_schnorr).
5. Sending the signed transaction to the Bitcoin network using the [bitcoin_send_transaction API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-method-bitcoin_send_transaction).

This canister's `send_from_${type}` endpoint returns the ID of the transaction
it sent to the network. You can track the status of this transaction using a
[block explorer](https://en.bitcoin.it/wiki/Block_chain_browser). Once the
transaction has at least one confirmation, you should be able to see it
reflected in your current balance.

## Conclusion

In this tutorial, you were able to:

* Deploy a canister smart contract on the ICP blockchain that can receive & send Bitcoin.
* Use a cycles faucet to deploy the canister to ICP blockchain on the mainnet for free.
* Connect the canister to the Bitcoin testnet.
* Send the canister some testnet BTC.
* Check the testnet BTC balance of the canister.
* Use the canister to send testnet BTC to another testnet BTC address. 

This example is extensively documented in the following tutorials:

* [Deploying your first Bitcoin dapp](https://internetcomputer.org/docs/current/samples/deploying-your-first-bitcoin-dapp).
* [Developing Bitcoin dapps locally](https://internetcomputer.org/docs/current/developer-docs/integrations/bitcoin/local-development).

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since the app e.g. offers a method to read balances.
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)

## Taproot transactions
In addition to P2PKH transactions, this example now also suppots P2TR
transactions, namely the so-called untweaked key path P2TR transactions, which
is the most efficient way of performing a P2TR transaction. The limitation of
this type of transactions is that it cannot be used in combination with scripts.
IMPORTANT: Note that BIP341 advises against using taproot addresses that can be
spent with an untweaked key. This precaution is to prevent attacks that can
occur when creating taproot multisigner addresses using specific multisignature
schemes. However, the Schnorr API of the internet computer does not support
Schnorr multisignatures.

This implementation has only been tested locally with regtest.
