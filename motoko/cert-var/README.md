# Certified Variable

![Compatibility](https://img.shields.io/badge/compatibility-0.7.0-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-cert-var-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-cert-var-example)

The example demonstrates the use of a single cryptographically certified variable, as supported by the Internet Computer. 



## Prerequisites

Verify the following before running this demo:

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

2. Install the front-end dependencies:

   ```text
   npm install
   ```

3. Build and deploy your canisters.

   ```text
   dfx deploy
   ```

## TO DO

- Set variable.
- Check variable.

## More info

General background:

- [Manage Canisters](https://sdk.dfinity.org/docs/developers-guide/working-with-canisters.html)
- [Quick  Start](https://sdk.dfinity.org/developers-guide/quickstart.html)
- [Developer's Guide](https://sdk.dfinity.org/developers-guide)
- [Language Reference](https://sdk.dfinity.org/language-guide)
