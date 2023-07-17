# Ledger transfer

## Overview
ICP transfer is a canister that can transfer ICP from its account to other accounts. It is an example of a canister that uses the ledger canister. Sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/ rust/ledger-transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/tokens_transfer).

## Architecture
The sample code revolves around one core transfer function which takes as input the amount of ICP to transfer, the account (and optionally the subaccount) to which to transfer ICP and returns either success or an error in case e.g. the ICP transfer canister doesn’t have enough ICP to do the transfer. In case of success, a unique identifier of the transaction is returned. This identifier will be stored in the memo of the transaction in the ledger.

This sample will use the Rust variant. 

## Prerequisites

This example requires an installation of:
- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Download and install [git.](https://git-scm.com/downloads)

### Step 1: Create a new `dfx` project and navigate into the project's directory.

```
dfx new --type=rust ledger_transfer
cd ledger_transfer
```

### Step 2: Determine ledger file locations

Go to the [releases overview](https://dashboard.internetcomputer.org/releases) and copy the latest replica binary revision. At the time of writing, this is `a17247bd86c7aa4e87742bf74d108614580f216d`.

The URL for the ledger WASM module is `https://download.dfinity.systems/ic/<REVISION>/canisters/ic-icrc1-ledger.wasm.gz`, so with the above revision it would be `https://download.dfinity.systems/ic/a17247bd86c7aa4e87742bf74d108614580f216d/canisters/ic-icrc1-ledger.wasm.gz`.

The URL for the ledger .did file is `https://raw.githubusercontent.com/dfinity/ic/<REVISION>/rs/rosetta-api/icrc1/ledger/ledger.did`, so with the above revision it would be `https://raw.githubusercontent.com/dfinity/ic/a17247bd86c7aa4e87742bf74d108614580f216d/rs/rosetta-api/icrc1/ledger/ledger.did`.

### Step 3: Configure the `dfx.json` file to use the ledger :

Replace its contents with this (but adapt the URLs to be the ones you determined in step 2:

```
{
  "canisters": {
    "ledger": {
      "type": "custom",
      "candid": "https://raw.githubusercontent.com/dfinity/ic/a17247bd86c7aa4e87742bf74d108614580f216d/rs/rosetta-api/icrc1/ledger/ledger.did",
      "wasm": "https://download.dfinity.systems/ic/a17247bd86c7aa4e87742bf74d108614580f216d/canisters/ic-icrc1-ledger.wasm.gz",
      "remote": {
        "id": {
          "ic": "ryjl3-tyaaa-aaaaa-aaaba-cai"
        }
      }
    },
    "ledger_transfer_backend": {
      "main": "src/ledger_transfer_backend/main.mo",
      "type": "motoko"
    },
    "ledger_transfer_frontend": {
      "dependencies": [
        "ledger_transfer_backend"
      ],
      "frontend": {
        "entrypoint": "src/ledger_transfer_frontend/src/index.html"
      },
      "source": [
        "src/ledger_transfer_frontend/assets",
        "dist/ledger_transfer_frontend/"
      ],
      "type": "assets"
    }
  },
  "defaults": {
     "replica": {
      "subnet_type":"system"
    },
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "output_env_file": ".env",
  "version": 1
}
```

### Step 5: Start a local replica:

```
dfx start --background
```

### Step 6: Create a new identity that will work as a minting account:

```
dfx identity new minter
dfx identity use minter
export MINT_ACC=$(dfx ledger account-id)
```

Transfers from the minting account will create Mint transactions. Transfers to the minting account will create Burn transactions.

### Step 7: Switch back to your default identity and record its ledger account identifier:

```
dfx identity use default
export LEDGER_ACC=$(dfx ledger account-id)
```


### Step 8: Deploy the ledger canister to your network:

```
dfx canister install ledger --argument "(variant {Init = record { token_name = \"NAME\"; token_symbol = \"SYMB\"; transfer_fee = 1000000; metadata = vec {}; minting_account = record {owner = principal \"$(dfx --identity minter identity get-principal)\";}; initial_balances = vec {}; archive_options = record {num_blocks_to_archive = 1000000; trigger_threshold = 1000000; controller_id = principal \"$(dfx identity get-principal)\"}; }})"
```

If you want to setup the ledger in a way that matches the production deployment, you should deploy it with archiving enabled. In this setup, the ledger canister dynamically creates new canisters to store old blocks. We recommend using this setup if you are planning to exercise the interface for fetching blocks.

### Step 9: Obtain the principal of the identity you use for development. 
This principal will be the controller of archive canisters.

```
dfx identity use default
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)
```

### Step 10: Deploy the ledger canister with archiving options:

```
dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}; archive_options = opt record { trigger_threshold = 2000; num_blocks_to_archive = 1000; controller_id = principal "'${ARCHIVE_CONTROLLER}'" }})'
```

If successful, the output should be:

```
Deployed canisters.
URLs:
  Frontend canister via browser
    ledger_transfer_frontend: http://127.0.0.1:4943/?canisterId=br5f7-7uaaa-aaaaa-qaaca-cai
  Backend canister via Candid interface:
    ledger: http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=bkyz2-fmaaa-aaaaa-qaaaq-cai
    ledger_transfer_backend: http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=be2us-64aaa-aaaaa-qaabq-cai
```

### Step 11: Verify that the ledger canister is healthy and working as expected by using the command:

```
dfx canister call ledger account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$LEDGER_ACC'")]) + "}")')' })'
```

The output should be:

```
(record { e8s = 100_000_000_000 : nat64 })
```

### Step 10: In a separate working directory, clone the Github repo containing the `ledger_transfer` project's files:

```
git clone https://github.com/dfinity/examples.git
```

### Step 11: Copy the files for the `ledger-transfer` canister into your `ledger_transfer` workspace:

```
cp -r ./examples/rust/ledger-transfer/src/* ./ledger_transfer/src
```

### Step 12: Edit your `dfx.json` file to include the following information within the 'canisters' section:

```
...
  "canisters": {
    "ledger_transfer": {
      "dependencies": [
        "ledger"
      ],
      "main": "src/ledger_transfer/main.mo",
      "type": "rust"
    },
    "ledger": {
      "type": "custom",
      "candid": "ledger.public.did",
      "wasm": "ledger.wasm"
    }
  },
...
```


### Step 13: Deploy this new canister:

```
dfx deploy
```

Your output should resemble the following:

```
Deployed canisters.
URLs:
  Frontend canister via browser
    ledger_transfer_frontend: http://127.0.0.1:4943/?canisterId=br5f7-7uaaa-aaaaa-qaaca-cai
  Backend canister via Candid interface:
    ledger: http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=bkyz2-fmaaa-aaaaa-qaaaq-cai
    ledger_transfer: http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=bw4dl-smaaa-aaaaa-qaacq-cai
    ledger_transfer_backend: http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=be2us-64aaa-aaaaa-qaabq-cai
```

### Step 14: Determine out the address of your canister:

```
dfx canister call ledger_transfer canisterAccount '()'
```

Your output should resemble the following:

```
(
  blob "\94\b9\bc]\ab(\ad\b93\8dE\19#\914\b6\a0\0e\dfam5\e4\e5\80\b5\01\9a~\e1_{",
)
```

### Step 15: Transfer funds to your canister:

```
dfx canister call ledger transfer '(record { to = blob "\08.\cf.?dz\c6\00\f4?8\a6\83B\fb\a5\b8\e6\8b\08_\02Y+w\f3\98\08\a8\d2\b5"; memo = 1; amount = record { e8s = 2_00_000_000 }; fee = record { e8s = 10_000 }; })'
```

If successful, the output should be:

```
(variant { Ok = 1 : nat64 })
```

### Step 16: Post a message as a new user:

```
dfx identity new --disable-encryption ALICE
dfx identity use ALICE
dfx canister call ledger_transfer post "(\"Test message\")"
```

### Step 17: Distribute rewards to users:

```
dfx identity use default
dfx canister call ledger_transfer distributeRewards '()'
```

## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#inter-canister-calls-and-rollbacks), since issues around inter-canister calls (here the ledger) can e.g. lead to time-of-check time-of-use or double spending security bugs.
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important financial data in the frontend that may be used by users to decide on future transactions. In this example, this is e.g. relevant for the call to `canisterBalance`. 
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralizing control is a fundamental aspect of decentralized finance applications.

