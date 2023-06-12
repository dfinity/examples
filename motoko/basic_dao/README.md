# Basic decentralized autonomous organization (DAO)

This sample project demonstrates a basic [decentralized autonomous organization](https://en.wikipedia.org/wiki/Decentralized_autonomous_organization) (DAO) that can be deployed to the [Internet Computer](https://github.com/dfinity/ic). The basic DAO sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/basic_dao) and [Rust](https://github.com/dfinity/examples/tree/master/rust/basic_dao). You can see a quick introduction on [YouTube](https://youtu.be/3IcYlieA-EE).

## Overview

A `basic_dao` can be initialized with a set of accounts: mappings from principal IDs to an amount of tokens. Account owners can query their account balance by calling `account_balance` and transfer tokens to other accounts by calling `transfer`. Anyone can call `list_accounts` to view all accounts.

Account owners can submit proposals by calling `submit_proposal`. A proposal specifies a canister, method and arguments for this method. Account owners can cast votes (either `Yes` or `No`) on a proposal by calling `vote`. The amount of votes cast is equal to amount of tokens the account owner has. If enough `Yes` votes are cast, `basic_dao` will execute the proposal by calling the proposal’s given method with the given args against the given canister. If enough `No` votes are cast, the proposal is not executed, and is instead marked as `Rejected`.

Certain system parameters, like the number of `Yes` votes needed to pass a proposal, can be queried by calling `get_system_params`. These system params can be modified via the proposal process, i.e. a proposal can be made to call `update_system_params` with updated values. The below demo does exactly that.

View the [canister service definition](https://github.com/dfinity/examples/blob/master/rust/basic_dao/src/basic_dao/src/basic_dao.did) for more details.
 
## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since this is a DAO's use case.
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since e.g. account_balance and list_accounts are query calls that a client may want to issue as update call.

### Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](../developer-docs/setup/install/index.mdx).
- [x] To run the test scripts, you need to download [ic-repl](https://github.com/chenyan2002/ic-repl/releases).

Begin by opening a terminal window.

 ### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

`cd basic_dao`
`dfx start --background`

 ### Step 2: Create test identities with the commands:

```
dfx identity new Alice --disable-encryption; dfx identity use Alice; export ALICE=$(dfx identity get-principal);
dfx identity new Bob --disable-encryption; dfx identity use Bob; export BOB=$(dfx identity get-principal);
```

 ### Step 3: Deploy `basic_dao` with initial test accounts.

```
dfx deploy --argument "(record {
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

 ### Step 4: Run the ic-repl test scripts:

```
ic-repl tests/account.test.sh
ic-repl tests/proposal.test.sh
```

