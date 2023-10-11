# Bitcoin Wallet Example

## Summary

This example dapp shows how to build a basic Bitcoin wallet making use of the
Internet Computer's [Bitcoin integration](https://smartcontracts.org/docs/developers-guide/concepts/bitcoin-integration.html).

## Step-by-step tutorial

Install the required node modules for the Bitcoin wallet webapp:

```bash
npm install
npm run build
npm run build-vanilla
```

While working on the Internet Computer does not require more configuration, working locally does. The additional instructions are provided in [the next section](#testing-locally).

Run the following commands to deploy and initialize the development Internet Identity canister and the Bitcoin wallet canister locally:

```bash
dfx deploy
dfx canister call bitcoin_wallet initialize
```

Run the following commands to deploy and initialize the Bitcoin wallet canister on the Internet Computer:

```bash
II_CANISTER_ID=identity dfx deploy --network ic bitcoin_wallet_assets
dfx canister --network ic call bitcoin_wallet initialize
```

## Testing locally

The Bitcoin wallet invokes the Bitcoin integration API through the management canister.
In order to test the Bitcoin wallet locally, follow the instructions below.

### Prerequisites

- [Bitcoin Core](https://bitcoin.org/en/download). Mac users are recommended to download the `.tar.gz` version.

The first step is to setup a local Bitcoin network.

### Setting up a local Bitcoin network

1. Unpack the `.tar.gz` file.

2. Create a directory named `data` inside the unpacked folder.

3. Create a file called `bitcoin.conf` at the root of the unpacked folder and add the following contents:

```
# Enable regtest mode. This is required to setup a private Bitcoin network.
regtest=1

# Dummy credentials that are required by `bitcoin-cli`.
rpcuser=btc-wallet
rpcpassword=Wjh4u6SAjT4UMJKxPmoZ0AN2r9qbE-ksXQ5I2_-Hm4w=
rpcauth=btc-wallet:8555f1162d473af8e1f744aa056fd728$afaf9cb17b8cf0e8e65994d1195e4b3a4348963b08897b4084d210e5ee588bcb
```

4. Run bitcoind to start the Bitcoin client using the following command:

```
./bin/bitcoind -conf=$(pwd)/bitcoin.conf -datadir=$(pwd)/data
```

5. Create a wallet:

```
./bin/bitcoin-cli -conf=$(pwd)/bitcoin.conf createwallet mywallet
```

If everything is setup correctly, you should see the following output:

```
{
  "name": "mywallet",
  "warning": ""
}
```

6. Generate a Bitcoin address and save it in a variable for later reuse:

```
export BTC_ADDRESS=$(./bin/bitcoin-cli -conf=$(pwd)/bitcoin.conf getnewaddress)
```

This will generate a Bitcoin address for your wallet to receive funds.

7. Mine blocks to receive some bitcoins as a reward.

```
./bin/bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 101 $BTC_ADDRESS
```

You should see an output that looks similar to, but not exactly like, the following:

```
[
  "1625281b2595b77276903868a0fe2fc31cb0c624e9bdc269e74a3f319ceb48de",
  "1cc5ba7e86fc313333c5448af6c7af44ff249eca3c8b681edc3c275efd3a2d38",
  "1d3c85b674497ba08a48d1b955bee5b4dc4505ffe4e9f49b428153e02e3e0764",
  ...
  "0dfd066985dc001ccc1fe6d7bfa53b7ad4944285dc173615792653bbd52151f1",
  "65975f1cd5809164f73b0702cf326204d8fee8b9669bc6bd510cb221cf09db5c",
]
```

### Synchronize blocks from bitcoind and create the canister

Synchronize blocks from bitcoind with the replica by executing the following command in the `bitcoin_wallet` folder:

```
dfx start
```

### Sending bitcoin to the Bitcoin wallet canister

To top up your Bitcoin wallet with bitcoins, get your address in the `Receive` tab of the website and run the following commands:

```
# Send a transaction that transfers 10 BTC to the provided Bitcoin address.
./bin/bitcoin-cli -conf=$(pwd)/bitcoin.conf -datadir=$(pwd)/data sendtoaddress BTC_ADDRESS 10 "" "" true true null "unset" null 1.1

# Mine 1 block that contains the transaction.
./bin/bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 1 $BTC_ADDRESS
```
