# Fetch ICP block

This example demonstrates querying ICP ledger blocks.

## Introduction

The application provides an interface exposes a single method, `fetch_block`.
This method fetches a single ledger block with the specified index on the mainnet.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have a wallet and enough cycles to deploy the canister on the mainnet.

## Demo

1. Reserve an identifier for your canister.

   ```text
   dfx canister --network ic create fetch_block
   ```

1. Deploy your canister.

   ```text
   dfx deploy --network ic fetch_block
   ```

1. Fetch a ledger block.

   ```text
    dfx canister --network ic call fetch_block fetch_block '(3_671_055)'
    (
      opt record {
        transaction = record {
          memo = 1_347_768_404 : nat64;
          operation = opt variant {
            Transfer = record {
              to = blob ";\a0\c7\9c\ee.Sn\85\c3t\da\16\a9\f1\f9\16\dc\e4\9d\11\f0[\17:\bfJ\1c\9e\fan\c3";
              fee = record { e8s = 10_000 : nat64 };
              from = blob "\9bgg\93b\98\8a\d8\9bYQ\22E\d8\15G\15y\a8q \cdK\27D\99\e3\ae\fal\a8\eb";
              amount = record { e8s = 100_000_000 : nat64 };
            }
          };
          created_at_time = record {
            timestamp_nanos = 1_653_423_511_396_865_000 : nat64;
          };
        };
        timestamp = record { timestamp_nanos = 1_653_423_514_702_069_961 : nat64 };
        parent_hash = opt blob "\d2\e3\cd\cav\e5\90\e5.q\0d)$+\fd\919:pj\f0he\a5J\d9\d9[P\cc\dc\dc";
      },
     )
    ```

