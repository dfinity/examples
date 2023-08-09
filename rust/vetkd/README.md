# vetKD System API Preview

This repository provides a canister (`src/system_api`) that offers the vetKD system API proposed in https://github.com/dfinity/interface-spec/pull/158, implemented in an **unsafe** manner **for demonstration purposes**.

Additionally, the repository provides:
* An example app backend canister (`src/app_backend`) implemented in **Rust** that makes use of this system API in order to provide caller-specific symmetric keys that can be used for AES encryption and decryption.

* An example frontend (`src/app_frontend_js`) that uses the backend from Javascript in the browser.

  The frontend uses the [ic-vetkd-utils](https://github.com/dfinity/ic/tree/master/packages/ic-vetkd-utils) to create a transport key pair that is used to obtain a verifiably-encrypted key from the system API, to decrypt this key, and to derive a symmetric key to be used for AES encryption/decryption.
  
  Because the `ic-vetkd-utils` are not yet published as NPM package at [npmjs.com](npmjs.com), a respective package file (`ic-vetkd-utils-0.1.0.tgz`) is included in this repository.

## Disclaimer

The implementation of [the proposed vetKD system API](https://github.com/dfinity/interface-spec/pull/158) used in this example is **unsafe**, e.g., we hard-code a master secret key, rather than using a master secret key that is distributed among sufficiently many Internet Computer nodes by means of distributed key generation. **Do not use this in production or for sensitive data**! This example is solely provided **for demonstration purposes** to collect feedback on the mentioned vetKD system API. See also the respective disclaimer [in the system API canister implementation](https://github.com/dfinity/examples/blob/master/rust/vetkd/src/system_api/src/lib.rs#L19-L26).

## Prerequisites
- [ ] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).
- [ ] Install [Node.js](https://nodejs.org/en/download/).
- [ ] Install [Rust](https://www.rust-lang.org/tools/install), and add Wasm as a target (`rustup target add wasm32-unknown-unknown`).

## Running Locally

1. Start a local internet computer.

   ```sh
   dfx start
   ```

1. Open a new terminal window.

1. Ensure the Canister SDK (dfx) uses the canister IDs that are hard-coded in the Rust source code:

   ```sh
   dfx canister create system_api --specified-id s55qq-oqaaa-aaaaa-aaakq-cai
   ```

   Without this, the Canister SDK (dfx) may use different canister IDs for the `system_api` and `app_backend` canisters in your local environment.

1. Ensure that the required node modules are available in your project directory, if needed, by running the following command:

   ```sh
   npm install
   ```

1. Register, build and deploy the project:

   ```sh
   dfx deploy
   ```

   This command should finish successfully with output similar to the following one:

   ```sh
   Deployed canisters.
   URLs:
   Frontend canister via browser
     app_frontend_js: http://127.0.0.1:4943/?canisterId=by6od-j4aaa-aaaaa-qaadq-cai
   Backend canister via Candid interface:
     app_backend: http://127.0.0.1:4943/?canisterId=avqkn-guaaa-aaaaa-qaaea-cai&id=tcvdh-niaaa-aaaaa-aaaoa-cai
     app_frontend: http://127.0.0.1:4943/?canisterId=avqkn-guaaa-aaaaa-qaaea-cai&id=b77ix-eeaaa-aaaaa-qaada-cai
     system_api: http://127.0.0.1:4943/?canisterId=avqkn-guaaa-aaaaa-qaaea-cai&id=s55qq-oqaaa-aaaaa-aaakq-cai
   ```

1. Open the printed URL for the `app_frontend_js` in your browser.
