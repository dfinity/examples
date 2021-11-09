# Ledger transfer

This example demonstrates an application that transfer ICPs to its most active users.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister SDK](https://smartcontracts.org).

*  You have stopped any Internet Computer or other network process that would create a port conflict on 8000.

## Demo

1. Start a local internet computer
   ```
   dfx start
   ```

1. Open a new terminal window

1. Build your canister
   ```
   dfx build
   ```

1. Install the ledger canister
   ```
dfx canister install --mode reinstall --argument '(record {send_whitelist=vec{}; minting_account="051b05839339f89053454a4b9865ea0452a4bffe2b1cd41f4982bad10c1e637c"; transaction_window = null; max_message_size_bytes = null; archive _options = null; initial_values = vec {record{"bdc4ee05d42cd0669786899f256c8fd7217fa71177bd1fa7b9534f568680a938"; record {e8s=100_000_000_000}}};})' ledger
   ```


1. Figure out the address of your canister
   ```
   dfx canister call ledger_transfer canisterAddress '()'
   ```

1. Transfer funds to your canister
   ```
   dfx canister call ledger transfer '(record { to = blob "\08.\cf.?dz\c6\00\f4?8\a6\83B\fb\a5\b8\e6\8b\08_\02Y+w\f3\98\08\a8\d2\b5"; memo = 1; amount = record { e8s = 2_00_000_000 }; fee = record { e8s = 10_000 }; })'
   ```

1. Post a message as a new user
   ```
   dfx identity new homer
   dfx identity use homer
   dfx canister call ledger_transfer post "(\"Nom Nom Love Donuts\")"
   ```

1. Distribute rewards to users
   ```
   dfx identity use default
   dfx canister call ledger_transfer distributeRewards '()'
   ```
