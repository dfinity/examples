# Basic DAO

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-dao-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-dao-example)

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
