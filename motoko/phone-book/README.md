# Phone book

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-phone-book-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-phone-book-example)

This example demonstrates a phone book application that is accessible from your
web browser.

## Introduction

The application is built from the following Motoko source code files:

*  `index.jsx`, which contains the JavaScript, React, and HTML used to generate
   the front-end user interface for the application when it is launched in a
   web browser; and

*  `Main.mo`, which contains the actor definition and methods exposed by this
   canister.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed [Node.js](https://nodejs.org).

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```text
   dfx canister create --all
   ```

1. Build your front-end.

   ```text
   npm install
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install --all
   ```

1. Take note of the URL at which the phone book is accessible.

   ```text
   echo "http://localhost:8000/?canisterId=$(dfx canister id www)"
   ```

1. Open the aforementioned URL in your web browser.
