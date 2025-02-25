# Token transfer


Token transfer is a canister that can transfer ICRC-1 tokens from its account to other accounts. It is an example of a canister that uses an ICRC-1 ledger canister. Sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/token_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/token_transfer).

## Architecture

The sample code revolves around one core transfer function which takes as input the amount of tokens to transfer, the `Account` to which to transfer tokens and returns either success or an error in case e.g. the token transfer canister doesn’t have enough tokens to do the transfer. In case of success, a unique identifier of the transaction is returned.

This sample will use the Rust variant.

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Step 1: Create a new `dfx` project and navigate into the project's directory

```bash
dfx new --type=rust token_transfer --no-frontend
cd token_transfer
```

## Step 2: Determine ICRC-1 ledger file locations

:::info 

You can read more about how to [setup the ICRC-1 ledger locally](https://internetcomputer.org/docs/current/developer-docs/defi/icrc-1/icrc1-ledger-setup).

:::

Go to the [releases overview](https://dashboard.internetcomputer.org/releases) and copy the latest replica binary revision.

The URL for the ledger Wasm module is `https://download.dfinity.systems/ic/<REVISION>/canisters/ic-icrc1-ledger.wasm.gz`.

The URL for the ledger .did file is `https://raw.githubusercontent.com/dfinity/ic/<REVISION>/rs/rosetta-api/icrc1/ledger/ledger.did`.

**OPTIONAL:**
If you want to make sure, you have the latest ICRC-1 ledger files you can run the following script.

```sh
curl -o download_latest_icrc1_ledger.sh "https://raw.githubusercontent.com/dfinity/ic/69988ae40e4cc0db7ef758da7dd5c0606075e926/rs/rosetta-api/scripts/download_latest_icrc1_ledger.sh"
chmod +x download_latest_icrc1_ledger.sh
./download_latest_icrc1_ledger.sh
```

## Step 3: Configure the `dfx.json` file to use the ledger

Replace its contents with this but adapt the URLs to be the ones you determined in step 2. Note that we are deploying the ICRC-1 ledger to the same canister id the ckBTC ledger uses on mainnet. This will make it easier to interact with it later.

```json
{
    "canisters": {
        "token_transfer_backend": {
            "candid": "src/token_transfer_backend/token_transfer_backend.did",
            "package": "token_transfer_backend",
            "type": "rust",
            "dependencies": ["icrc1_ledger_canister"]
        },
        "icrc1_ledger_canister": {
            "type": "custom",
            "candid": "https://raw.githubusercontent.com/dfinity/ic/<REVISION>/rs/rosetta-api/icrc1/ledger/ledger.did",
            "wasm": "https://download.dfinity.systems/ic/<REVISION>/canisters/ic-icrc1-ledger.wasm.gz",
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

```json
...
"candid": icrc1_ledger.did,
"wasm" : icrc1_ledger.wasm.gz,
  ...
```

## Step 4: Start a local instance of the Internet Computer

```bash
dfx start --background --clean
```

## Step 5: Create a new identity that will work as a minting account

```bash
dfx identity new minter --storage-mode plaintext
dfx identity use minter
export MINTER=$(dfx identity get-principal)
```

:::info

Transfers from the minting account will create Mint transactions. Transfers to the minting account will create Burn transactions.

:::

## Step 6: Switch back to your default identity

Record its principal to mint an initial balance to when deploying the ledger:

```bash
dfx identity use default
export DEFAULT=$(dfx identity get-principal)
```

## Step 7: Deploy the ICRC-1 ledger locally

Take a moment to read the details of the call made below. Not only are you deploying an ICRC-1 ledger canister, you are also:

-   Setting the minting account to the principal you saved in a previous step (`MINTER`)
-   Minting 100 tokens to the DEFAULT principal
-   Setting the transfer fee to 0.0001 tokens
-   Naming the token Local ICRC1 / L-ICRC1

```bash
dfx deploy icrc1_ledger_canister --argument "(variant { Init =
record {
     token_symbol = \"ICRC1\";
     token_name = \"L-ICRC1\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = 10_000;
     metadata = vec {};
     initial_balances = vec { record { record { owner = principal \"${DEFAULT}\"; }; 10_000_000_000; }; };
     archive_options = record {
         num_blocks_to_archive = 1000;
         trigger_threshold = 2000;
         controller_id = principal \"${MINTER}\";
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

## Step 8: Verify that the ledger canister is healthy and working as expected

> [!TIP]
> You can find more information on how to [interact with the ICRC-1 ledger](https://internetcomputer.org/docs/current/developer-docs/defi/icrc-1/using-icrc1-ledger#icrc-1-and-icrc-1-extension-endpoints)

````bash
dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
  owner = principal \"${DEFAULT}\";
  }
)"
```

The output should be:

```bash
(10_000_000_000 : nat)
````

## Step 9: Prepare the token transfer canister

Replace the contents of the `src/token_transfer_backend/Cargo.toml` file with the following:

```toml
[package]
name = "token_transfer_backend"
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
serde_derive = "1.0.197"
```

Replace the contents of the `src/token_transfer_backend/src/lib.rs` file with the following:

```rust
use candid::{CandidType, Deserialize, Principal};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};
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

    let transfer_args: TransferArg = TransferArg {
        // can be used to distinguish between transactions
        memo: None,
        // the amount we want to transfer
        amount: args.amount,
        // we want to transfer tokens from the default subaccount of the canister
        from_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: args.to_account,
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
    };

    // 1. Asynchronously call another canister function using `ic_cdk::call`.
    ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        // 2. Convert a textual representation of a Principal into an actual `Principal` object. The principal is the one we specified in `dfx.json`.
        //    `expect` will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
            .expect("Could not decode the principal."),
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc1_transfer",
        // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
        (transfer_args,),
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

Replace the contents of the `src/token_transfer_backend/token_transfer_backend.did` file with the following:

> [!TIP]
> The `token_transfer_backend.did` file is a Candid file that describes the service interface of the canister. It was generated from the Rust code using the `candid-extractor` tool. You can read more about the [necessary steps](https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid).

```did
type Account = record { owner : principal; subaccount : opt vec nat8 };
type Result = variant { Ok : nat; Err : text };
type TransferArgs = record { to_account : Account; amount : nat };
service : { transfer : (TransferArgs) -> (Result) }

```

## Step 10: Deploy the token transfer canister

```bash
dfx deploy token_transfer_backend
```

## Step 11: Transfer funds to your canister

> [!WARNING]
> Make sure that you are using the default `dfx` account that we minted tokens to in step 7 for the following steps.

Make the following call to transfer 10 tokens to the canister:

```bash
dfx canister call icrc1_ledger_canister icrc1_transfer "(record {
  to = record {
    owner = principal \"$(dfx canister id token_transfer_backend)\";
  };
  amount = 1_000_000_000;
})"
```

If successful, the output should be:

```bash
(variant { Ok = 1 : nat })
```

## Step 12: Transfer funds from the canister

Now that the canister owns tokens on the ledger, you can transfer 1 token from the canister to another account, in this case back to the default account:

```bash
dfx canister call token_transfer_backend transfer "(record {
  amount = 100_000_000;
  to_account = record {
    owner = principal \"$(dfx identity get-principal)\";
  };
})"
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

-   [Inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview), since issues around inter-canister calls (here the ledger) can e.g. lead to time-of-check time-of-use or double spending security bugs.
-   [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important financial data in the frontend that may be used by users to decide on future transactions. In this example, this is e.g. relevant for the call to `canisterBalance`.
-   [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview), since decentralizing control is a fundamental aspect of decentralized finance applications.
