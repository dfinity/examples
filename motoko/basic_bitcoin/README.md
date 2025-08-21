# Basic Bitcoin

This tutorial will walk you through how to deploy a sample [canister smart contract](https://wiki.internetcomputer.org/wiki/Canister_smart_contract) **that can send and receive Bitcoin** on the Internet Computer.

## Architecture

This example internally leverages the [ECDSA
API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key),
[Schnorr API](https://internetcomputer.org/docs/current/references/ic-interface-spec#ic-sign_with_schnorr), and [Bitcoin
API](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md)
of the Internet Computer.

For a deeper understanding of the ICP < > BTC integration, see the [Bitcoin integration documentation](https://internetcomputer.org/docs/current/developer-docs/multi-chain/bitcoin/overview).

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/motoko/daily_planner)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.

## Using the basic_bitcoin example

### Step 1: Generating a Bitcoin address

Bitcoin has different types of addresses (e.g. P2PKH, P2SH, P2TR). You may want
to check [this
article](https://bitcoinmagazine.com/technical/bitcoin-address-types-compared-p2pkh-p2sh-p2wpkh-and-more)
if you are interested in a high-level comparison of different address types.
These addresses can be generated from an ECDSA public key or a Schnorr
([BIP340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki),
[BIP341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)) public
key. The example code showcases how your canister can generate and spend from
three types of addresses:
1. A [P2PKH address](https://en.bitcoin.it/wiki/Transaction#Pay-to-PubkeyHash)
   using the
   [ecdsa_public_key](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-method-ecdsa_public_key)
   API.
2. A [P2TR
   address](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)
   where the funds can be spent using the internal key only ([P2TR key path
   spend with unspendable script
   tree](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki#cite_note-23)).
3. A [P2TR
   address](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)
   where the funds can be spent using either 1) the internal key or 2) the
   provided public key with the script path, where the Merkelized Alternative
   Script Tree (MAST) consists of a single script allowing to spend funds by
   exactly one key.

On the Candid UI of your canister, click the "Call" button under
`get_${type}_address` to generate a `${type}` Bitcoin address, where `${type}`
is one of `[p2pkh, p2tr_key_only, p2tr]` (corresponding to the three types of
addresses described above, in the same order).

Or, if you prefer the command line:

```bash
dfx canister --network=ic call basic_bitcoin get_${type}_address
```

* We are generating a Bitcoin testnet address, which can only be
used for sending/receiving Bitcoin on the Bitcoin testnet.

### Step 2: Receiving bitcoin

Now that the canister is deployed and you have a Bitcoin address, it's time to receive
some testnet bitcoin. You can use one of the Bitcoin faucets, such as [coinfaucet.eu](https://coinfaucet.eu),
to receive some bitcoin. **Make sure to choose Bitcoin testnet4!**

Enter your address and click on "Get bitcoins!". In the example below we will use Bitcoin address `n31eU1K11m1r58aJMgTyxGonu7wSMoUYe7`, but you will use your address. The Bitcoin address you see will be different from the one above
because the ECDSA/Schnorr public key your canister retrieves is unique.

Once the transaction has at least one confirmation, which takes ten minutes on average,
you'll be able to see it in your canister's balance.

### Step 3: Checking your bitcoin balance

You can check a Bitcoin address's balance by using the `get_balance` endpoint on your canister.

In the Candid UI, paste in your canister's address, and click on "Call".

Alternatively, make the call using the command line. Be sure to replace `mheyfRsAQ1XrjtzjfU1cCH2B6G1KmNarNL` with your own generated address:

```bash
dfx canister --network=ic call basic_bitcoin get_balance '("mheyfRsAQ1XrjtzjfU1cCH2B6G1KmNarNL")'
```

Checking the balance of a Bitcoin address relies on the [bitcoin_get_balance](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md#bitcoin_get_balance) API.

### Step 4: Sending bitcoin

You can send bitcoin using the `send_from_${type}` endpoint on your canister, where
`${type}` is one of
`[p2pkh_address, p2tr_key_only_address, p2tr_address_key_path, p2tr_address_script_path]`.

In the Candid UI, add a destination address and an amount to send. In the example
below, we're sending 4'321 Satoshi (0.00004321 BTC) back to the testnet faucet.

Via the command line, the same call would look like this:

```bash
dfx canister --network=ic call basic_bitcoin send_from_p2pkh_address '(record { destination_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt"; amount_in_satoshi = 4321; })'
```

The `send_from_${type}` endpoint can send bitcoin by:

1. Getting the percentiles of the most recent fees on the Bitcoin network using the [bitcoin_get_current_fee_percentiles API](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md#bitcoin_get_current_fee_percentiles).
2. Fetching your unspent transaction outputs (UTXOs), using the [bitcoin_get_utxos API](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md#bitcoin_get_utxos).
3. Building a transaction, using some of the UTXOs from step 2 as input and the destination address and amount to send as output.
   The fee percentiles obtained from step 1 are used to set an appropriate fee.
4. Signing the inputs of the transaction using the
   [sign_with_ecdsa
   API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-method-sign_with_ecdsa)/\
   [sign_with_schnorr](https://org5p-7iaaa-aaaak-qckna-cai.icp0.io/docs#ic-sign_with_schnorr).
5. Sending the signed transaction to the Bitcoin network using the [bitcoin_send_transaction API](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md#bitcoin_send_transaction).

This canister's `send_from_${type}` endpoint returns the ID of the transaction
it sent to the network. You can track the status of this transaction using a
[block explorer](https://en.bitcoin.it/wiki/Block_chain_browser). Once the
transaction has at least one confirmation, you should be able to see it
reflected in your current balance.

### Step 5: Retrieving block headers

You can also get a range of Bitcoin block headers by using the `get_block_headers`
endpoint on your canister.

In the Candid UI, write the desired start height and optionally end height, and click on "Call":

Alternatively, make the call using the command line. Be sure to replace `10` with your desired start height:

```bash
dfx canister --network=ic call basic_bitcoin get_block_headers "(10: nat32)"
```
or replace `0` and `11` with your desired start and end height respectively:
```bash
dfx canister --network=ic call basic_bitcoin get_block_headers "(0: nat32, 11: nat32)"
```
