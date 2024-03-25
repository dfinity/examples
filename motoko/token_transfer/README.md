# Token transfer

[View this samples code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/token_transfer).

## Overview

Token transfer is a canister that can transfer ICRC-1 tokens from its account to other accounts. It is an example of a canister that uses an ICRC-1 ledger canister. Sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/token_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/token_transfer).

## Architecture

The sample code revolves around one core transfer function which takes as input the amount of tokens to transfer, the `Account` to which to transfer tokens and returns either success or an error in case e.g. the token transfer canister doesn’t have enough tokens to do the transfer. In case of success, a unique identifier of the transaction is returned.

This sample will use the Motoko variant.

## Prerequisites

This example requires an installation of:

-   [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
-   [x] Download and install [git](https://git-scm.com/downloads).

## How to get there

The following steps will guide you through the process of setting up the token transfer canister for your own project.

> [!TIP]
> If you just want to interact with this example, follow steps 4-8 and 10-12 below.

### Step 1: Create a new `dfx` project and navigate into the project's directory.

```bash
dfx new --type=motoko token_transfer --no-frontend
cd token_transfer
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

Replace its contents with this but adapt the URLs to be the ones you determined in step 2:

```json
{
    "canisters": {
        "token_transfer_backend": {
            "main": "src/token_transfer_backend/main.mo",
            "type": "motoko",
            "dependencies": ["icrc1_ledger_canister"]
        },
        "icrc1_ledger_canister": {
            "type": "custom",
            "candid": "https://raw.githubusercontent.com/dfinity/ic/d87954601e4b22972899e9957e800406a0a6b929/rs/rosetta-api/icrc1/ledger/ledger.did",
            "wasm": "https://download.dfinity.systems/ic/d87954601e4b22972899e9957e800406a0a6b929/canisters/ic-icrc1-ledger.wasm.gz"
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

### Step 4: Start a local replica:

```bash
dfx start --background --clean
```

### Step 5: Create a new identity that will work as a minting account:

```bash
dfx identity new minter --storage-mode plaintext
dfx identity use minter
export MINTER=$(dfx identity get-principal)
```

> [!IMPORTANT]
> Transfers from the minting account will create Mint transactions. Transfers to the minting account will create Burn transactions.

### Step 6: Switch back to your default identity and record its principal to mint an initial balance to when deploying the ledger:

```bash
dfx identity use default
export DEFAULT=$(dfx identity get-principal)
```

### Step 7: Deploy the ICRC-1 ledger locally:

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

### Step 8: Verify that the ledger canister is healthy and working as expected by using the command:

> [!NOTE]
> You can find more information on how to interact with the ICRC-1 ledger [here](https://internetcomputer.org/docs/current/developer-docs/defi/icrc-1/using-icrc1-ledger#icrc-1-and-icrc-1-extension-endpoints)

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

### Step 9: Prepare the token transfer canister:

Replace the contents of the `src/token_transfer_backend/main.mo` file with the following:

```motoko
import Icrc1Ledger "canister:icrc1_ledger_canister";
import Debug "mo:base/Debug";
import Result "mo:base/Result";
import Option "mo:base/Option";
import Blob "mo:base/Blob";
import Error "mo:base/Error";

actor {

  type Account = {
    owner : Principal;
    subaccount : ?[Nat8];
  };

  type TransferArgs = {
    amount : Nat;
    toAccount : Account;
  };

  public shared ({ caller }) func transfer(args : TransferArgs) : async Result.Result<Icrc1Ledger.BlockIndex, Text> {
    Debug.print(
      "Transferring "
      # debug_show (args.amount)
      # " tokens to account"
      # debug_show (args.toAccount)
    );

    let transferArgs : Icrc1Ledger.TransferArg = {
      // can be used to distinguish between transactions
      memo = null;
      // the amount we want to transfer
      amount = args.amount;
      // we want to transfer tokens from the default subaccount of the canister
      from_subaccount = null;
      // if not specified, the default fee for the canister is used
      fee = null;
      // we take the principal and subaccount from the arguments and convert them into an account identifier
      to = args.toAccount;
      // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
      created_at_time = null;
    };

    try {
      // initiate the transfer
      let transferResult = await Icrc1Ledger.icrc1_transfer(transferArgs);

      // check if the transfer was successfull
      switch (transferResult) {
        case (#Err(transferError)) {
          return #err("Couldn't transfer funds:\n" # debug_show (transferError));
        };
        case (#Ok(blockIndex)) { return #ok blockIndex };
      };
    } catch (error : Error) {
      // catch any errors that might occur during the transfer
      return #err("Reject message: " # Error.message(error));
    };
  };
};

```

### Step 10: Deploy the token transfer canister:

```bash
dfx deploy token_transfer_backend
```

### Step 11: Transfer funds to your canister:

> [!IMPORTANT]
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

### Step 12: Transfer funds from the canister:

Now that the canister owns tokens on the ledger, you can transfer 1 token from the canister to another account, in this case back to the default account:

```bash
dfx canister call token_transfer_backend transfer "(record {
  amount = 100_000_000;
  toAccount = record {
    owner = principal \"$(dfx identity get-principal)\";
  };
})"
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

-   [Inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#inter-canister-calls-and-rollbacks), since issues around inter-canister calls (here the ledger) can e.g. lead to time-of-check time-of-use or double spending security bugs.
-   [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important financial data in the frontend that may be used by users to decide on future transactions. In this example, this is e.g. relevant for the call to `canisterBalance`.
-   [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralizing control is a fundamental aspect of decentralized finance applications.
