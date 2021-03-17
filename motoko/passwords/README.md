# Design Pattern: Usernames & Passwords

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-password-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-password-example)

This example demonstrates how a more traditional username and password authentication scheme can be used alongside the `caller` functionality within a Motoko actor.

## Overview

End-users of an application might want to access their account from multiple devices, a friend's device, or a shared device like at a library. While the Internet Computer provides authentication with every request in the form of the `caller` principal identifier, it can be useful to associate that principal with an existing account through the common pattern of a username and password.

## Implementation

The back-end canister exposes several methods for the lifecycle of a user account. A new user can `signup` with a specified username and password (which are then stored in a secure way). An existing user can `login` to associate a new device to an existing account or `logout` to remove that association. An authenticated user can ask `whoami` to get the currently authenticated account.

Note: There are many obvious improvements (storing accounts in a more optimal data structure, implementing a real hashing algorithm) but hopefully this example illustrates the concepts in a simple way.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

*  You have installed the front-end dependencies by running `npm install`.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Create, build, and install both canisters.

   ```text
   dfx deploy
   ```

1. Create a new account.

   ```text
   dfx canister call passwords signup 'record { username = "Alice"; password = "p4ssw0rd" }'
   ```

   Note: You should probably not be sending passwords in the clear like this, despite messages to the Internet Computer being sent over HTTPS. Again, just trying to illustrate a concept.

1. Authenticate your command line identity.

   ```text
   dfx canister call passwords login 'record { username = "Alice"; password = "p4ssw0rd" };
   ```

1. Confirm that you're logged in.

  ```text
  dfx canister call passwords whoami
  ```

1. Observe the result.

  ```text
  ("Alice")
  ```

1. Navigate in your browser to the canister's front-end.

1. Enter your username and password. Then click "Who am I?"

1. Observe the result.

  ```text
  Logged in as: Alice
  ```
