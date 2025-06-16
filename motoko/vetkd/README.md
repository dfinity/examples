# vetKD API

This example demonstrates how to use the Internet Computer's **vetKeys** feature to:

1. Derive a (symmetric) cryptographic AES-GCM-256 key *in the user's browser*, and use it there for encryption and decryption.
2. Use identity-based encryption (IBE) to encrypt some plaintext for a particular *principal*, derive a respective decryption key *in the user's browser* for the user that is currently logged in, and use it to decrypt some ciphertext.

It includes:

* An example app backend canister (`src/app_backend`) implemented in **Motoko** that provides caller-specific symmetric keys that can be used for AES encryption and decryption.

* An example frontend (`src/app_frontend_js`) that uses the backend from Javascript in the browser.

  The frontend uses [@dfinity/vetkeys](https://www.npmjs.com/package/@dfinity/vetkeys) ([docs](https://dfinity.github.io/vetkeys/modules/_dfinity_vetkeys.html)) to create a transport key pair that is used to obtain a verifiably encrypted key from the system API, to decrypt this key, and to derive a symmetric key to be used for AES encryption/decryption.

## Prerequisites
- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`
- [x] Install [Node.js](https://nodejs.org/en/download/).

Begin by opening a terminal window.

## Step 1: Setup the project environment

```bash
cd examples/motoko/vetkd
dfx start --background --clean
```

## Step 2: Set up dependencies for a Node.js

Ensure that the required node modules are available in your project directory, if needed, by running the following command:

```sh
npm install
```

## Step 3: Register, build, and deploy the project:

```sh
dfx deploy
```

This command should finish successfully with output similar to the following one:

```sh
Deployed canisters.
URLs:
  Frontend canister via browser:
    app_frontend_js:
      - http://xobql-2x777-77774-qaaja-cai.localhost:4943/ (Recommended)
      - http://127.0.0.1:4943/?canisterId=xobql-2x777-77774-qaaja-cai (Legacy)
    internet_identity:
      - http://xjaw7-xp777-77774-qaajq-cai.localhost:4943/ (Recommended)
      - http://127.0.0.1:4943/?canisterId=xjaw7-xp777-77774-qaajq-cai (Legacy)
  Backend canister via Candid interface:
    app_backend: http://127.0.0.1:4943/?canisterId=x4hhs-wh777-77774-qaaka-cai&id=xhc3x-m7777-77774-qaaiq-cai
    internet_identity: http://127.0.0.1:4943/?canisterId=x4hhs-wh777-77774-qaaka-cai&id=xjaw7-xp777-77774-qaajq-cai
```

## Step 4: Open frontend

Open the URL for the `app_frontend_js` (printed in the previous step) in your browser.