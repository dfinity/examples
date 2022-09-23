# Basic DAO

This sample project demonstrates a basic DAO 
([Decentralized Autonomous Organization](https://en.wikipedia.org/wiki/Decentralized_autonomous_organization)) 
that can be deployed to the [Internet Computer](https://github.com/dfinity/ic).

## Overview

A `basic_dao` can be initialized with a set of accounts: mappings from principal IDs to an amount of tokens. 
Account owners can query their account balance by calling `account_balance` and transfer tokens to other
accounts by calling `transfer`. Anyone can call `list_accounts` to view all accounts. 

Account owners can submit proposals by calling `submit_proposal`. A proposal specifies a canister, method 
and arguments for this method. Account owners can cast votes (either `yes` or `no`) on a proposal by calling `vote`.
The amount of votes cast is equal to amount of tokens the account owner has. If enough `yes` votes are cast,
`basic_dao` will execute the proposal by calling the proposal's given method with the given args against the given
canister. If enough `no` votes are cast, the proposal is not executed, and is instead marked as `rejected`.

Certain system parameters, like the number of `yes` votes needed to pass a proposal, can be queried by calling
`get_system_params`. These system params can be modified via the proposal process, i.e. a proposal can be
made to call `update_system_params` with updated values.

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since this is a DAO's use case.
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since e.g. account_balance and list_accounts are query calls that a client may want to issue as update call.

## Prerequisites

Verify the following before running this demo:

* You have downloaded and installed the [DFINITY Canister SDK](https://sdk.dfinity.org).

* To run the test scripts, you need to download [ic-repl](https://github.com/chenyan2002/ic-repl/releases).

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Create test identities

   ```text
   $ dfx identity new Alice --disable-encryption; dfx identity use Alice; export ALICE=$(dfx identity get-principal);
   $ dfx identity new Bob --disable-encryption; dfx identity use Bob; export BOB=$(dfx identity get-principal);
   ```

1. Deploy `basic_dao` with initial accounts.

   ```text
   $ dfx deploy --argument "(record {
    accounts = vec { record { owner = principal \"$ALICE\"; tokens = record { amount_e8s = 100_000_000 }; };
                     record { owner = principal \"$BOB\"; tokens = record { amount_e8s = 100_000_000 };}; };
    proposals = vec {};
    system_params = record {
        transfer_fee = record { amount_e8s = 10_000 };
        proposal_vote_threshold = record { amount_e8s = 10_000_000 };
        proposal_submission_deposit = record { amount_e8s = 10_000 };
    };
   })"
   ```

1. Run the `ic-repl` test script.

   ```text
   ic-repl tests/account.test.sh
   ic-repl tests/proposal.test.sh
   ```

## Rust implementation

An equivalent interface is implemented in Rust as well, see [Basic DAO example in Rust](https://github.com/dfinity/examples/tree/master/rust/basic_dao).
The interface is not exactly the same due to the ergonomic differences between Motoko and Rust.

* Variant tags are capitalized in Rust.
* `Proposal.voters` has type `Vec<Principal>` in Rust, while we use `List<Principal>` in Motoko for easier appending of voters.
* Token `amount_e8s` has type `u64` in Rust, and `Nat` in Motoko.
