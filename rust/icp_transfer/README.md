# ICP transfer

[View this samples code on GitHub](https://github.com/dfinity/examples/tree/master/rust/icp_transfer).

## Overview

ICP transfer is a canister that can transfer ICP from its account to other accounts. It is an example of a canister that uses the ledger canister. Sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/icp_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/icp_transfer).

> [!NOTE]
> The ICP ledger supports the ICRC1 standard, which is the recommended standard for token transfers. You can read more about the differences [here](https://internetcomputer.org/docs/current/developer-docs/defi/overview) and find an example of how to transfer ICRC1 tokens from a canister in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/token_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/token_transfer).

## Architecture

The sample code revolves around one core transfer function which takes as input the amount of ICP to transfer, the account (and optionally the subaccount) to which to transfer ICP and returns either success or an error in case e.g. the ICP transfer canister doesn’t have enough ICP to do the transfer. In case of success, a unique identifier of the transaction is returned. This identifier will be stored in the memo of the transaction in the ledger.

This sample will use the Rust variant.

## Prerequisites

This example requires an installation of:

-   [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
-   [x] Download and install [git.](https://git-scm.com/downloads)

## How to get there

The following steps will guide you through the process of setting up the token transfer canister for your own project.

> [!TIP]
> If you just want to interact with this example, follow steps 4-8 and 10-13 below.

### Step 1: Create a new `dfx` project and navigate into the project's directory.

```bash
dfx new --type=rust icp_transfer --no-frontend
cd icp_transfer
```

### Step 2: Determine ledger file locations

> [!NOTE]
> You can read more about how to setup the ICP ledger locally [here](https://internetcomputer.org/docs/current/developer-docs/defi/icp-tokens/ledger-local-setup).

Go to the [releases overview](https://dashboard.internetcomputer.org/releases) and copy the latest replica binary revision. At the time of writing, this is `d87954601e4b22972899e9957e800406a0a6b929`.

The URL for the ledger Wasm module is `https://download.dfinity.systems/ic/<REVISION>/canisters/ledger-canister.wasm.gz`, so with the above revision it would be `https://download.dfinity.systems/ic/d87954601e4b22972899e9957e800406a0a6b929/canisters/ledger-canister.wasm.gz`.

The URL for the ledger.did file is `https://raw.githubusercontent.com/dfinity/ic/<REVISION>/rs/rosetta-api/icp_ledger/ledger.did`, so with the above revision it would be `https://raw.githubusercontent.com/dfinity/ic/d87954601e4b22972899e9957e800406a0a6b929/rs/rosetta-api/icp_ledger/ledger.did`.

[OPTIONAL]
If you want to make sure you have the latest ICP ledger files, you can run the following script. Please ensure that you have [`jq`](https://jqlang.github.io/jq/) installed as the script relies on it.

```sh
curl -o download_latest_icp_ledger.sh "https://raw.githubusercontent.com/dfinity/ic/00a4ab409e6236d4082cee4a47544a2d87b7190d/rs/rosetta-api/scripts/download_latest_icp_ledger.sh"
chmod +x download_latest_icp_ledger.sh
./download_latest_icp_ledger.sh
```

### Step 3: Configure the `dfx.json` file to use the ledger :

Replace its contents with this but adapt the URLs to be the ones you determined in step 2:

```json
{
    "canisters": {
        "icp_transfer_backend": {
            "candid": "src/icp_transfer_backend/icp_transfer_backend.did",
            "package": "icp_transfer_backend",
            "type": "rust"
        },
        "icp_ledger_canister": {
            "type": "custom",
            "candid": "https://raw.githubusercontent.com/dfinity/ic/d87954601e4b22972899e9957e800406a0a6b929/rs/rosetta-api/icp_ledger/ledger.did",
            "wasm": "https://download.dfinity.systems/ic/d87954601e4b22972899e9957e800406a0a6b929/canisters/ledger-canister.wasm.gz",
            "remote": {
                "id": {
                    "ic": "ryjl3-tyaaa-aaaaa-aaaba-cai"
                }
            }
        }
    },
    "defaults": {
        "build": {
            "args": "",
            "packtool": ""
        }
    },
    "output_env_file": ".env",
    "version": 1
}
```

### Step 4: Start a local replica:

```bash
dfx start --background --clean
```

### Step 5: Create a new identity that will work as a minting account:

```bash
dfx identity new minter --storage-mode plaintext
dfx identity use minter
export MINTER_ACCOUNT_ID=$(dfx ledger account-id)
```

> [!IMPORTANT]
> Transfers from the minting account will create Mint transactions. Transfers to the minting account will create Burn transactions.

### Step 6: Switch back to your default identity and record its ledger account identifier:

```bash
dfx identity use default
export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)
```

### Step 7: Deploy the ledger canister to your network:

Take a moment to read the details of the call made below. Not only are you deploying the ICP ledger canister, you are also:

-   Deploying the canister to the same canister ID as the mainnet ledger canister. This is to make it easier to switch between local and mainnet deployments and set in `dfx.json` using `specified_id`.
-   Setting the minting account to the account identifier you saved in a previous step (MINTER_ACCOUNT_ID).
-   Minting 100 ICP tokens to the DEFAULT_ACCOUNT_ID (1 ICP is equal to 10^8 e8s, hence the name).
-   Setting the transfer fee to 0.0001 ICP.
-   Naming the token Local ICP / LICP

```bash
dfx deploy icp_ledger_canister --argument "
  (variant {
    Init = record {
      minting_account = \"$MINTER_ACCOUNT_ID\";
      initial_values = vec {
        record {
          \"$DEFAULT_ACCOUNT_ID\";
          record {
            e8s = 10_000_000_000 : nat64;
          };
        };
      };
      send_whitelist = vec {};
      transfer_fee = opt record {
        e8s = 10_000 : nat64;
      };
      token_symbol = opt \"LICP\";
      token_name = opt \"Local ICP\";
    }
  })
"
```

If successful, the output should be:

```bash
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    icp_ledger_canister: http://127.0.0.1:4943/?canisterId=bnz7o-iuaaa-aaaaa-qaaaa-cai&id=ryjl3-tyaaa-aaaaa-aaaba-cai
```

### Step 8: Verify that the ledger canister is healthy and working as expected by using the command:

```bash
dfx canister call icp_ledger_canister account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$DEFAULT_ACCOUNT_ID'")]) + "}")')'})'
```

The output should be:

```bash
(record { e8s = 10_000_000_000 : nat64 })
```

### Step 9: Prepare the token transfer canister:

Replace the contents of the `src/icp_transfer_backend/Cargo.toml` file with the following:

```toml
[package]
name = "icp_transfer_backend"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib"]

[dependencies]
candid = "0.10.4"
ic-cdk = "0.12.1"
ic-cdk-macros = "0.8.4"
ic-ledger-types = "0.9.0"
serde = "1.0.197"
serde_derive = "1.0.197"

```

Replace the contents of the `src/icp_transfer_backend/src/lib.rs` file with the following:

```rust
use candid::{CandidType, Principal};

use ic_cdk_macros::*;
use ic_ledger_types::{
    AccountIdentifier, BlockIndex, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    amount: Tokens,
    to_principal: Principal,
    to_subaccount: Option<Subaccount>,
}

#[update]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to principal {} subaccount {:?}",
        &args.amount,
        &args.to_principal,
        &args.to_subaccount
    );
    let to_subaccount = args.to_subaccount.unwrap_or(DEFAULT_SUBACCOUNT);
    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: args.amount,
        fee: Tokens::from_e8s(10_000),
        // The subaccount of the account identifier that will be used to withdraw tokens and send them
        // to another account identifier. If set to None then the default subaccount will be used.
        // See the [Ledger doc](https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/#accounts).
        from_subaccount: None,
        to: AccountIdentifier::new(&args.to_principal, &to_subaccount),
        created_at_time: None,
    };
    ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, transfer_args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .map_err(|e| format!("ledger transfer error {:?}", e))
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();

```

Replace the contents of the `src/icp_transfer_backend/icp_transfer_backend.did` file with the following:

> [!NOTE]
> The `icp_transfer_backend.did` file is a Candid file that describes the service interface of the canister. It was generated from the Rust code using the `candid-extractor` tool. You can read more about the necessary steps [here](https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid).

```did
type Result = variant { Ok : nat64; Err : text };
type Tokens = record { e8s : nat64 };
type TransferArgs = record {
  to_principal : principal;
  to_subaccount : opt vec nat8;
  amount : Tokens;
};
service : {
  canister_account : () -> (vec nat8) query;
  transfer : (TransferArgs) -> (Result);
}

```

### Step 10: Deploy the token transfer canister:

```bash
dfx deploy icp_transfer_backend
```

### Step 11: Determine out the address of your canister:

```bash
TOKENS_TRANSFER_ACCOUNT_ID="$(dfx ledger account-id --of-canister icp_transfer_backend)"
TOKENS_TRANSFER_ACCOUNT_ID_BYTES="$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$TOKENS_TRANSFER_ACCOUNT_ID'")]) + "}")')"
```

### Step 12: Transfer funds to your canister:

> [!IMPORTANT]
> Make sure that you are using the default `dfx` account that we minted tokens to in step 7 for the following steps.

Make the following call to transfer funds to the canister:

```bash
dfx canister call icp_ledger_canister transfer "(record { to = ${TOKENS_TRANSFER_ACCOUNT_ID_BYTES}; memo = 1; amount = record { e8s = 2_00_000_000 }; fee = record { e8s = 10_000 }; })"
```

If successful, the output should be:

```bash
(variant { Ok = 1 : nat64 })
```

### Step 13: Transfer funds from the canister:

Now that the canister owns ICP on the ledger, you can transfer funds from the canister to another account, in this case back to the default account:

```bash
dfx canister call icp_transfer_backend transfer "(record { amount = record { e8s = 1_00_000_000 }; to_principal = principal \"$(dfx identity get-principal)\"})"
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

-   [Inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#inter-canister-calls-and-rollbacks), since issues around inter-canister calls (here the ledger) can e.g. lead to time-of-check time-of-use or double spending security bugs.
-   [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important financial data in the frontend that may be used by users to decide on future transactions. In this example, this is e.g. relevant for the call to `canisterBalance`.
-   [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralizing control is a fundamental aspect of decentralized finance applications.
