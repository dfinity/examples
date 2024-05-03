# ICP transfer

[View this samples code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/icp_transfer).

## Overview

ICP transfer is a canister that can transfer ICP from its account to other accounts. It is an example of a canister that uses the ledger canister. Sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/icp_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/icp_transfer).

> [!NOTE]
> The ICP ledger supports the ICRC1 standard, which is the recommended standard for token transfers. You can read more about the differences [here](https://internetcomputer.org/docs/current/developer-docs/defi/overview) and find an example of how to transfer ICRC1 tokens from a canister in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/token_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/token_transfer).

## Architecture

The sample code revolves around one core transfer function which takes as input the amount of ICP to transfer, the account (and optionally the subaccount) to which to transfer ICP and returns either success or an error in case e.g. the ICP transfer canister doesn’t have enough ICP to do the transfer. In case of success, a unique identifier of the transaction is returned. This identifier will be stored in the memo of the transaction in the ledger.

This sample will use the Motoko variant.

## Prerequisites

This example requires an installation of:

-   [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
-   [x] Download and install [git](https://git-scm.com/downloads).

## How to get there

The following steps will guide you through the process of setting up the token transfer canister for your own project.

> [!TIP]
> If you just want to interact with this example, follow steps 4-6 and 8-11 below.

### Step 1: Create a new `dfx` project and navigate into the project's directory.

```bash
dfx dfx new --type=motoko icp_transfer --no-frontend
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

> [!IMPORTANT]
> Don't forget to add the `icp_ledger_canister` as a dependency for `icp_transfer_backend`, otherwise the build will fail.

```json
{
    "canisters": {
        "icp_transfer_backend": {
            "main": "src/icp_transfer_backend/main.mo",
            "type": "motoko",
            "dependencies": ["icp_ledger_canister"]
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

### Step 5: Deploy the ledger canister to your network:

> [!IMPORTANT]
> Transfers from the minting account will create Mint transactions. Transfers to the minting account will create Burn transactions.

Take a moment to read the details of the call made below. Not only are you deploying the ICP ledger canister, you are also:

-   Deploying the canister to the same canister ID as the mainnet ledger canister. This is to make it easier to switch between local and mainnet deployments and set in `dfx.json` using `specified_id`.
-   Setting the minting account to the anonymous principal (`2vxsx-fae`)
-   Minting 100 ICP tokens to the DEFAULT_ACCOUNT_ID (1 ICP is equal to 10^8 e8s, hence the name).
-   Setting the transfer fee to 0.0001 ICP.
-   Naming the token Local ICP / LICP

```bash
dfx deploy icp_ledger_canister --argument "(variant {
    Init = record {
      minting_account = \"$(dfx ledger --identity anonymous account-id)\";
      initial_values = vec {
        record {
          \"$(dfx ledger --identity default account-id)\";
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

### Step 6: Verify that the ledger canister is healthy and working as expected by using the command:

```bash
dfx canister call icp_ledger_canister account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$(dfx ledger --identity default account-id)'")]) + "}")')'})'
```

The output should be:

```bash
(record { e8s = 10_000_000_000 : nat64 })
```

### Step 7: Prepare the ICP transfer canister:

Replace the contents of the `src/icp_transfer_backend/main.mo` file with the following:

```motoko
import IcpLedger "canister:icp_ledger_canister";
import Debug "mo:base/Debug";
import Result "mo:base/Result";
import Option "mo:base/Option";
import Blob "mo:base/Blob";
import Error "mo:base/Error";
import Array "mo:base/Array";
import Principal "mo:base/Principal";

actor {
  type Tokens = {
    e8s : Nat64;
  };

  type TransferArgs = {
    amount : Tokens;
    toPrincipal : Principal;
    toSubaccount : ?Blob;
  };

  public shared ({ caller }) func transfer(args : TransferArgs) : async Result.Result<IcpLedger.BlockIndex, Text> {
    Debug.print(
      "Transferring "
      # debug_show (args.amount)
      # " tokens to principal "
      # debug_show (args.toPrincipal)
      # " subaccount "
      # debug_show (args.toSubaccount)
    );

    let transferArgs : IcpLedger.TransferArgs = {
      // can be used to distinguish between transactions
      memo = 0;
      // the amount we want to transfer
      amount = args.amount;
      // the ICP ledger charges 10_000 e8s for a transfer
      fee = { e8s = 10_000 };
      // we are transferring from the canisters default subaccount, therefore we don't need to specify it
      from_subaccount = null;
      // we take the principal and subaccount from the arguments and convert them into an account identifier
      to = Blob.toArray(Principal.toLedgerAccount(args.toPrincipal, args.toSubaccount));
      // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
      created_at_time = null;
    };

    try {
      // initiate the transfer
      let transferResult = await IcpLedger.transfer(transferArgs);

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

### Step 8: Deploy the token transfer canister:

```bash
dfx deploy icp_transfer_backend
```

### Step 9: Determine out the address of your canister:

```bash
TOKENS_TRANSFER_ACCOUNT_ID="$(dfx ledger account-id --of-canister icp_transfer_backend)"
TOKENS_TRANSFER_ACCOUNT_ID_BYTES="$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$TOKENS_TRANSFER_ACCOUNT_ID'")]) + "}")')"
```

### Step 10: Transfer funds to your canister:

> [!IMPORTANT]
> Make sure that you are using the default `dfx` account that we minted tokens to in step 7 for the following steps.

Make the following call to transfer funds to the canister:

```bash
dfx canister --identity default call icp_ledger_canister transfer "(record { to = ${TOKENS_TRANSFER_ACCOUNT_ID_BYTES}; memo = 1; amount = record { e8s = 2_00_000_000 }; fee = record { e8s = 10_000 }; })"
```

If successful, the output should be:

```bash
(variant { Ok = 1 : nat64 })
```

### Step 11: Transfer funds from the canister:

Now that the canister owns ICP on the ledger, you can transfer funds from the canister to another account, in this case back to the default account:

```bash
dfx canister call icp_transfer_backend transfer "(record { amount = record { e8s = 100_000_000 }; toPrincipal = principal \"$(dfx identity --identity default get-principal)\"})"
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

-   [Inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#inter-canister-calls-and-rollbacks), since issues around inter-canister calls (here the ledger) can e.g. lead to time-of-check time-of-use or double spending security bugs.
-   [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important financial data in the frontend that may be used by users to decide on future transactions.
-   [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralizing control is a fundamental aspect of decentralized finance applications.
