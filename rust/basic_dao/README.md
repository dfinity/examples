# Basic DAO

This sample project demonstrates a basic DAO 
([Decentralized Autonomous Organization](https://en.wikipedia.org/wiki/Decentralized_autonomous_organization)) 
that can be deployed to the [Internet Computer](https://github.com/dfinity/ic).

## Overview

A `basic_dao` can be initialized with a set of accounts: mappings from principal IDs to an amount of tokens. 
Account owners can query their account balance by calling `account_balance` and transfer tokens to other
accounts by calling `transfer`. Anyone can call `list_accounts` to view all accounts. 

Account owners can submit proposals by calling `submit_proposal`. A proposal specifies a canister, method 
and arguments for this method. Account owners can cast votes (either `Yes` or `No`) on a proposal by calling `vote`. 
The amount of votes cast is equal to amount of tokens the account owner has. If enough `Yes` votes are cast, 
`basic_dao` will execute the proposal by calling the proposal's given method with the given args against the given 
canister. If enough `No` votes are cast, the proposal is not executed, and is instead marked as `Rejected`.

Certain system parameters, like the number of `Yes` votes needed to pass a proposal, can be queried by calling 
`get_system_params`. These system params can be modified via the proposal process, i.e. a proposal can be
made to call `update_system_params` with updated values. The below demo does exactly that.

View the [canister service definition](src/basic_dao/src/basic_dao.did) for a more details.

## Prerequisites

Verify the following before running this demo:

* You have installed the Rust toolchain (e.g. cargo)

* You have installed [didc](https://github.com/dfinity/candid/tree/master/tools/didc)

* You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

* You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Build the `basic_dao` canister.

   ```text
   $ make clean; make
   ```

2. Start a local internet computer.

   ```text
   $ dfx start
   ```

3. Open a new terminal window.
   
4. Create test identities

   ```text
   $ dfx identity new Alice; dfx identity use Alice; export ALICE=$(dfx identity get-principal); 
   $ dfx identity new Bob; dfx identity use Bob; export BOB=$(dfx identity get-principal); 
   ```

5. Deploy `basic_dao` with initial accounts.

   ```text
   $ dfx deploy --argument "(record {
    accounts = vec { record { owner = principal \"$ALICE\"; tokens = record { amount_e8s = 100_000_000:nat64 }; }; 
                     record { owner = principal \"$BOB\"; tokens = record { amount_e8s = 100_000_000:nat64 };}; };
    proposals = vec {};
    system_params = record {
        transfer_fee = record { amount_e8s = 10_000:nat64 };
        proposal_vote_threshold = record { amount_e8s = 10_000_000:nat64 };
        proposal_submission_deposit = record { amount_e8s = 10_000:nat64 };
    };
   })"
   ```

6. List accounts and confirm you see 2 accounts

   ```text
   $ dfx canister call basic_dao list_accounts '()'
   ```

7. Call `account_balance` as `Bob`.

   ```text
   $ dfx canister call basic_dao account_balance '()'
   ```
   You should see as output:

   ```text
   (record { amount_e8s = 100_000_000 : nat64 })
   ```
   
8. Transfer tokens to `Alice`:

   ```text
   $ dfx canister call basic_dao transfer "(record { to = principal \"$ALICE\"; amount = record { amount_e8s = 90_000_000:nat64;};})";
   ```
   Output:
   ```text
   (variant { Ok })
   ```

9. List accounts and see that the transfer was made:

   ```text
   $ dfx canister call basic_dao list_accounts '()'
   ```
   Output:
   ```text
    (
      vec {
        record {
          owner = principal "$ALICE";
          tokens = record { amount_e8s = 190_000_000 : nat64 };
        };
        record {
          owner = principal "$BOB";
          tokens = record { amount_e8s = 9_990_000 : nat64 };
        };
      },
    )
    ```
    Note that the transfer fee was deducted from Bob's account
   
10. Let's make a proposal to change the transfer fee. We can call `get_system_params` to learn the current transfer fee:
   ```text
   $ dfx canister call basic_dao get_system_params '()';
   ```

   Output:
   ```text
   (
     record {
       transfer_fee = record { amount_e8s = 10_000 : nat64 };
       proposal_vote_threshold = record { amount_e8s = 10_000_000 : nat64 };
       proposal_submission_deposit = record { amount_e8s = 10_000 : nat64 };
     },
   )
   ```

   To change `transfer_fee`, we need to submit a proposal by calling `submit_proposal`, which takes a `ProposalPayload` as an arg:
   ```text
   type ProposalPayload = record {
     canister_id: principal;
     method: text;
     message: blob;
   };
   ```
   
   We can change `transfer_fee` by calling basic_dao's `update_system_params` method. This method takes
   a `UpdateSystemParamsPayload` as an arg, which we need to encode into a `blob` to use in `ProposalPayload`.
   Use `didc` to encode a `UpdateSystemParamsPayload`:

   ```text
   $ didc encode '(record { transfer_fee = opt record { amount_e8s = 20_000:nat64; }; })' -f blob
   ```
   Output:
   ```text
   blob "DIDL\03l\01\f2\c7\94\ae\03\01n\02l\01\b9\ef\93\80\08x\01\00\01 N\00\00\00\00\00\00"
   ```
   
   We can then submit the proposal:
   ```text
   $ dfx canister call basic_dao submit_proposal '(record { canister_id = principal "rrkah-fqaaa-aaaaa-aaaaq-cai";
   method = "update_system_params":text;
   message = blob "DIDL\03l\01\f2\c7\94\ae\03\01n\02l\01\b9\ef\93\80\08x\01\00\01 N\00\00\00\00\00\00"; })'
   ```
   
   Note the output proposal ID:
   ```text
   (variant { Ok = 0 : nat64 })
   ```
   
   Confirm the proposal was created:
   ```text
   $ dfx canister call basic_dao get_proposal '(0:nat64)'
   ```
   You should see `state = variant { Open };` in the output.

   Vote on the proposal:
   ```text
   $ dfx canister call basic_dao vote '(record { proposal_id = 0:nat64; vote = variant { Yes };})'
   ```
   
   You should see the following output:
   ```text
   (variant { Ok = variant { Open } })
   ```
   
   Because we voted as `Bob`, and `Bob` does not have enough voting power to pass proposals, the proposal remains `Open`.
   To get the proposal accepted, we can vote with `Alice`:
   ```text
   $ dfx identity use Alice; dfx canister call basic_dao vote '(record { proposal_id = 0:nat64; vote = variant { Yes };})';
   ```
   
   You should see the following output:
   ```text
   (variant { Ok = variant { Accepted } })
   ```

   Query the proposal again:
   ```text
   $ dfx canister call basic_dao get_proposal '(0:nat64)'
   ```
   And see that the state is `Succeeded`:
   ```text
   state = variant { Succeeded };
   ```
   
   Query the system params again and see that `transfer_fee` has been updated:
   ```text
   $ dfx canister call basic_dao get_system_params '()'
   ```
   Output:
   ```text
   (
     record {
       transfer_fee = record { amount_e8s = 20_000 : nat64 };
       proposal_vote_threshold = record { amount_e8s = 10_000_000 : nat64 };
       proposal_submission_deposit = record { amount_e8s = 10_000 : nat64 };
     },
   )
   ```
   