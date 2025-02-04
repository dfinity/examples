# Token transfer_from

`token_transfer_from_backend` is a canister that can transfer ICRC-1 tokens on behalf of accounts to other accounts. It is an example of a canister that uses an ICRC-1 ledger canister that supports the [ICRC-2](https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-2) approve and transfer from standard. Sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/token_transfer_from) and [Rust](https://github.com/dfinity/examples/tree/master/rust/token_transfer_from).

## Architecture

The sample code revolves around one core transfer function which takes as input the amount of tokens to transfer, the `Account` to which to transfer tokens and returns either success or an error in case e.g. the token transfer canister doesn’t have enough tokens to do the transfer or the caller has not approved the canister to spend their tokens. In case of success, a unique identifier of the transaction is returned. The example code assumes the caller of `transfer` has already approved the token transfer canister to spend their tokens.

This sample will use the Rust variant.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).

Begin by opening a terminal window.

## Step 1: Setup the project environment

Start a local instance of the Internet Computer and create a new project with the commands:

```bash
dfx start --background
dfx new --type=motoko token_transfer --no-frontend
cd token_transfer
```

## Step 2: Determine ICRC-1 ledger file locations

> [!TIP]
> [Learn more about how to setup the ICRC-1 ledger locally](https://internetcomputer.org/docs/current/developer-docs/defi/icrc-1/icrc1-ledger-setup)

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

Replace its contents with this but adapt the URLs to be the ones you determined in step 2:

```json
{
    "canisters": {
        "token_transfer_from_backend": {
            "main": "src/token_transfer_from_backend/main.mo",
            "type": "motoko",
            "dependencies": ["icrc1_ledger_canister"]
        },
        "icrc1_ledger_canister": {
            "type": "custom",
            "candid": "https://raw.githubusercontent.com/dfinity/ic/<REVISION>/rs/rosetta-api/icrc1/ledger/ledger.did",
            "wasm": "https://download.dfinity.systems/ic/<REVISION>/canisters/ic-icrc1-ledger.wasm.gz"
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

## Step 4: Use the anonymous identity as the minting account

```bash
export MINTER=$(dfx --identity anonymous identity get-principal)
```

> [!TIP]
> Transfers from the minting account will create Mint transactions. Transfers to the minting account will create Burn transactions.


## Step 5: Record your default identity's principal to mint an initial balance to when deploying the ledger

```bash
export DEFAULT=$(dfx identity get-principal)
```

## Step 6: Deploy the ICRC-1 ledger locally


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

## Step 7: Verify that the ledger canister is healthy and working as expected by using the command

> [!TIP]
> [Learn more about how to interact with the ICRC-1 ledger](https://internetcomputer.org/docs/current/developer-docs/defi/icrc-1/using-icrc1-ledger#icrc-1-and-icrc-1-extension-endpoints).

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

## Step 8: Prepare the token transfer canister

Replace the contents of the `src/token_transfer_from_backend/main.mo` file with the following:

```motoko
import Icrc1Ledger "canister:icrc1_ledger_canister";
import Debug "mo:base/Debug";
import Result "mo:base/Result";
import Error "mo:base/Error";

actor {

  type TransferArgs = {
    amount : Nat;
    toAccount : Icrc1Ledger.Account;
  };

  public shared ({ caller }) func transfer(args : TransferArgs) : async Result.Result<Icrc1Ledger.BlockIndex, Text> {
    Debug.print(
      "Transferring "
      # debug_show (args.amount)
      # " tokens to account"
      # debug_show (args.toAccount)
    );

    let transferFromArgs : Icrc1Ledger.TransferFromArgs = {
      // the account we want to transfer tokens from (in this case we assume the caller approved the canister to spend funds on their behalf)
      from = {
        owner = caller;
        subaccount = null;
      };
      // can be used to distinguish between transactions
      memo = null;
      // the amount we want to transfer
      amount = args.amount;
      // the subaccount we want to spend the tokens from (in this case we assume the default subaccount has been approved)
      spender_subaccount = null;
      // if not specified, the default fee for the canister is used
      fee = null;
      // we take the principal and subaccount from the arguments and convert them into an account identifier
      to = args.toAccount;
      // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
      created_at_time = null;
    };

    try {
      // initiate the transfer
      let transferFromResult = await Icrc1Ledger.icrc2_transfer_from(transferFromArgs);

      // check if the transfer was successful
      switch (transferFromResult) {
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

## Step 9: Deploy the token transfer canister

```bash
dfx deploy token_transfer_from_backend
```

## Step 10: Approve the canister to transfer funds on behalf of the user

:::info

Make sure that you are using the default `dfx` account that we minted tokens to in step 6 for the following steps.

:::

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

## Step 11: Let the canister transfer funds on behalf of the user

Now that the canister has an approval for the `default` identities tokens on the ledger, the canister can transfer 1 token on behalf of the `default` identity to another account, in this case to the canisters own account.

```bash
dfx canister call token_transfer_from_backend transfer "(record {
  amount = 100_000_000;
  toAccount = record {
    owner = principal \"$(dfx canister id token_transfer_from_backend)\";
  };
})"
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

-   [Inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview), since issues around inter-canister calls (here the ledger) can e.g. lead to time-of-check time-of-use or double spending security bugs.
-   [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important financial data in the frontend that may be used by users to decide on future transactions.
-   [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview), since decentralizing control is a fundamental aspect of decentralized finance applications.
