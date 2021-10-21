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

4. Start a local web server hosting the front end.

   ```text
   npm start
   ```

5. Visit the frontend, and do the demo there:

   http://localhost:8080/

   Should present an entry for "New value of variable",
   and a button to "Set and get!".

   Enter a number and click the button.

   The canister updates its certificate, and the frontend checks it.

## More info

General background:

- [Manage Canisters](https://sdk.dfinity.org/docs/developers-guide/working-with-canisters.html)
- [Quick  Start](https://sdk.dfinity.org/developers-guide/quickstart.html)
- [Developer's Guide](https://sdk.dfinity.org/developers-guide)
- [Language Reference](https://sdk.dfinity.org/language-guide)
