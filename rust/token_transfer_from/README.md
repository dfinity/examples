# Token transfer_from

[View this samples code on GitHub](https://github.com/dfinity/examples/tree/master/rust/token_transfer_from).

## Overview

`token_transfer_from_backend` is a canister that can transfer ICRC-1 tokens on behalf of accounts to other accounts. It is an example of a canister that uses an ICRC-1 ledger canister that supports the [ICRC-2](https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-2) approve and transfer from standard. Sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/token_transfer_from) and [Rust](https://github.com/dfinity/examples/tree/master/rust/token_transfer_from).

## Architecture

The sample code revolves around one core transfer function which takes as input the amount of tokens to transfer, the `Account` to which to transfer tokens and returns either success or an error in case e.g. the token transfer canister doesn’t have enough tokens to do the transfer or the caller has not approved the canister to spend their tokens. In case of success, a unique identifier of the transaction is returned. The example code assumes the caller of `transfer` has already approved the token transfer canister to spend their tokens.

This sample will use the Rust variant.

## Prerequisites

This example requires an installation of:

-   [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
-   [x] Download and install [git.](https://git-scm.com/downloads)

## How to get there

The following steps will guide you through the process of setting up the token transfer canister for your own project.

> [!TIP]
> If you just want to interact with this example, follow steps 4-6 and 8-10 below.

### Step 1: Create a new `dfx` project and navigate into the project's directory.

```bash
dfx new --type=rust token_transfer_from --no-frontend
cd token_transfer_from
```

### Step 2: Determine ICRC-1 ledger file locations

> [!NOTE]
> You can read more about how to setup the ICRC-1 ledger locally [here](https://internetcomputer.org/docs/current/developer-docs/defi/icrc-1/icrc1-ledger-setup).

Go to the [releases overview](https://dashboard.internetcomputer.org/releases) and copy the latest replica binary revision. At the time of writing, this is `d87954601e4b22972899e9957e800406a0a6b929`.

The URL for the ledger Wasm module is `https://download.dfinity.systems/ic/<REVISION>/canisters/ic-icrc1-ledger.wasm.gz`, so with the above revision it would be `https://download.dfinity.systems/ic/d87954601e4b22972899e9957e800406a0a6b929/canisters/ic-icrc1-ledger.wasm.gz`.

The URL for the ledger .did file is `https://raw.githubusercontent.com/dfinity/ic/<REVISION>/rs/rosetta-api/icrc1/ledger/ledger.did`, so with the above revision it would be `https://raw.githubusercontent.com/dfinity/ic/d87954601e4b22972899e9957e800406a0a6b929/rs/rosetta-api/icrc1/ledger/ledger.did`.

**OPTIONAL:**
If you want to make sure, you have the latest ICRC-1 ledger files you can run the following script.

```sh
curl -o download_latest_icrc1_ledger.sh "https://raw.githubusercontent.com/dfinity/ic/326df23607fc8280a047daba2d8462f1dfc57466/rs/rosetta-api/scripts/download_latest_icrc1_ledger.sh"
chmod +x download_latest_icrc1_ledger.sh
./download_latest_icrc1_ledger.sh
```

### Step 3: Configure the `dfx.json` file to use the ledger :

Replace its contents with this but adapt the URLs to be the ones you determined in step 2. Note that we are deploying the ICRC-1 ledger to the same canister id the ckBTC ledger uses on mainnet. This will make it easier to interact with it later.

```json
{
    "canisters": {
        "token_transfer_from_backend": {
            "candid": "src/token_transfer_from_backend/token_transfer_from_backend.did",
            "package": "token_transfer_from_backend",
            "type": "rust"
        },
        "icrc1_ledger_canister": {
            "type": "custom",
            "candid": "https://raw.githubusercontent.com/dfinity/ic/d87954601e4b22972899e9957e800406a0a6b929/rs/rosetta-api/icrc1/ledger/ledger.did",
            "wasm": "https://download.dfinity.systems/ic/d87954601e4b22972899e9957e800406a0a6b929/canisters/ic-icrc1-ledger.wasm.gz",
            "specified_id": "mxzaz-hqaaa-aaaar-qaada-cai"
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

If you chose to download the ICRC-1 ledger files with the script, you need to replace the Candid and Wasm file entries:

```
...
"candid": icrc1_ledger.did,
"wasm" : icrc1_ledger.wasm.gz,
...
```

### Step 4: Start a local replica:

```bash
dfx start --background --clean
```

### Step 5: Deploy the ICRC-1 ledger locally:

> [!IMPORTANT]
> Transfers from the `minting_account` will create Mint transactions. Transfers to the minting account will create Burn transactions.

Take a moment to read the details of the call made below. Not only are you deploying an ICRC-1 ledger canister, you are also:

-   Setting the minting account to the anonymous principal (`2vxsx-fae`)
-   Minting 100 tokens to the default identity
-   Setting the transfer fee to 0.0001 tokens
-   Naming the token Local ICRC1 / L-ICRC1
-   Enabling the ICRC-2 standard for the ledger

```bash
dfx deploy icrc1_ledger_canister --argument "(variant {
  Init = record {
    token_symbol = \"ICRC1\";
    token_name = \"L-ICRC1\";
    minting_account = record {
      owner = principal \"$(dfx identity --identity anonymous get-principal)\"
    };
    transfer_fee = 10_000;
    metadata = vec {};
    initial_balances = vec {
      record {
        record {
          owner = principal \"$(dfx identity --identity default get-principal)\";
        };
        10_000_000_000;
      };
    };
    archive_options = record {
      num_blocks_to_archive = 1000;
      trigger_threshold = 2000;
      controller_id = principal \"$(dfx identity --identity anonymous get-principal)\";
    };
    feature_flags = opt record {
      icrc2 = true;
    };
  }
})"
```

If successful, the output should be:

```bash
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    icrc1_ledger_canister: http://127.0.0.1:4943/?canisterId=bnz7o-iuaaa-aaaaa-qaaaa-cai&id=mxzaz-hqaaa-aaaar-qaada-cai
```

### Step 6: Verify that the ledger canister is healthy and working as expected by using the command:

> [!NOTE]
> You can find more information on how to interact with the ICRC-1 ledger [here](https://internetcomputer.org/docs/current/developer-docs/defi/icrc-1/using-icrc1-ledger#icrc-1-and-icrc-1-extension-endpoints)

````bash
dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
  owner = principal \"$(dfx identity --identity default get-principal)\";
})"
```

The output should be:

```bash
(10_000_000_000 : nat)
````

### Step 7: Prepare the token transfer canister:

Replace the contents of the `src/token_transfer_from_backend/Cargo.toml` file with the following:

```toml
[package]
name = "token_transfer_from_backend"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib"]

[dependencies]
candid = "0.10"
ic-cdk = "0.12"
ic-cdk-timers = "0.6" # Feel free to remove this dependency if you don't need timers
icrc-ledger-types = "0.1.5"
serde = "1.0.197"
```

Replace the contents of the `src/token_transfer_from_backend/src/lib.rs` file with the following:

```rust
use candid::{CandidType, Deserialize, Principal};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArgs {
    amount: NumTokens,
    to_account: Account,
}

#[ic_cdk::update]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let transfer_from_args = TransferFromArgs {
        // the account we want to transfer tokens from (in this case we assume the caller approved the canister to spend funds on their behalf)
        from: Account::from(ic_cdk::caller()),
        // can be used to distinguish between transactions
        memo: None,
        // the amount we want to transfer
        amount: args.amount,
        // the subaccount we want to spend the tokens from (in this case we assume the default subaccount has been approved)
        spender_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: args.to_account,
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
    };

    // 1. Asynchronously call another canister function using `ic_cdk::call`.
    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        // 2. Convert a textual representation of a Principal into an actual `Principal` object. The principal is the one we specified in `dfx.json`.
        //    `expect` will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
            .expect("Could not decode the principal."),
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc2_transfer_from",
        // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
        (transfer_from_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply `map_err` to transform any network or system errors encountered during the call into a more readable string format.
    //    The `?` operator is then used to propagate errors: if the result is an `Err`, it returns from the function with that error,
    //    otherwise, it unwraps the `Ok` value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the `Result<BlockIndex, TransferError>`, for further processing.
    .0
    // 8. Use `map_err` again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();

```

Replace the contents of the `src/token_transfer_from_backend/token_transfer_from_backend.did` file with the following:

> [!NOTE]
> The `token_transfer_from.did` file is a Candid file that describes the service interface of the canister. It was generated from the Rust code using the `candid-extractor` tool. You can read more about the necessary steps [here](https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid).

```did
type Account = record { owner : principal; subaccount : opt blob };
type Result = variant { Ok : nat; Err : text };
type TransferArgs = record { to_account : Account; amount : nat };
service : { transfer : (TransferArgs) -> (Result) }
```

### Step 8: Deploy the token transfer canister:

```bash
dfx deploy token_transfer_from_backend
```

### Step 9: Approve the canister to transfer funds on behalf of the user:

> [!IMPORTANT]
> Make sure that you are using the default `dfx` account that we minted tokens to in step 5 for the following steps.

Make the following call to approve the `token_transfer_from_backend` canister to transfer 100 tokens on behalf of the `default` identity:

```bash
dfx canister call --identity default icrc1_ledger_canister icrc2_approve "(
  record {
    spender= record {
      owner = principal \"$(dfx canister id token_transfer_from_backend)\";
    };
    amount = 10_000_000_000: nat;
  }
)"
```

If successful, the output should be:

```bash
(variant { Ok = 1 : nat })
```

### Step 10: Let the canister transfer funds on behalf of the user:

Now that the canister has an approval for the `default` identities tokens on the ledger, the canister can transfer 1 token on behalf of the `default` identity to another account, in this case to the canisters own account.

```bash
dfx canister call token_transfer_from_backend transfer "(record {
  amount = 100_000_000;
  to_account = record {
    owner = principal \"$(dfx canister id token_transfer_from_backend)\";
  };
})"
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

-   [Inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#inter-canister-calls-and-rollbacks), since issues around inter-canister calls (here the ledger) can e.g. lead to time-of-check time-of-use or double spending security bugs.
-   [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important financial data in the frontend that may be used by users to decide on future transactions.
-   [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralizing control is a fundamental aspect of decentralized finance applications.
