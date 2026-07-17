# VetKey Password Manager (Rust)

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/vetkeys/password_manager)

Also available in: [Motoko](../../../motoko/vetkeys/password_manager)

The **VetKey Password Manager** is an example application demonstrating how to use **VetKeys** and **Encrypted Maps** to build a secure, decentralized password manager on the **Internet Computer (IC)**. This application allows users to create password vaults, store encrypted passwords, and share vaults with other users via their **Internet Identity Principal**.

## Features

- **Secure Password Storage**: Uses VetKey to encrypt passwords before storing them in Encrypted Maps.
- **Vault-Based Organization**: Users can create multiple vaults, each containing multiple passwords.
- **Access Control**: Vaults can be shared with other users via their **Internet Identity Principal**.

## Build and deploy from the command line

### Prerequisites

- Install [Node.js](https://nodejs.org/en/download/)
- Install [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- Install the [Rust toolchain](https://www.rust-lang.org/tools/install), then add the WASM target: `rustup target add wasm32-unknown-unknown`

### (Optionally) choose a different master key

This example uses `test_key_1` by default. To use a different [available master key](https://docs.internetcomputer.org/concepts/vetkeys/#api-overview), change the `init_args` value in `icp.yaml` before deploying.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/vetkeys/password_manager
```

### Deploy

```bash
icp network start -d
icp deploy
```

Open the frontend URL printed by `icp deploy`.

To run the frontend in development mode with hot reloading (after `icp deploy`):

```bash
npm run dev
```

When done, stop the local network to free up the port for other projects:

```bash
icp network stop
```

## Example components

### Backend (`backend/`)

An **Encrypted Maps**-enabled Rust canister that securely stores passwords.

> **Note.** This backend is hand-written today. An upstream Rust macro that generates an entire Encrypted Maps canister in one line — `ic_vetkeys::export_encrypted_maps_canister!(...)` — is in progress ([dfinity/vetkeys#404](https://github.com/dfinity/vetkeys/pull/404)); it produces the same Candid interface with far less boilerplate.

### Frontend (`frontend/`)

A **Svelte** application providing a user-friendly interface for managing vaults and passwords. It talks to the backend through the `@icp-sdk/vetkeys` Encrypted Maps client.

## Updating the Candid interface

`backend/backend.did` defines the backend's public interface. If you change the backend's public API, regenerate it:

```bash
icp build backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > backend/backend.did
```

## Limitations

This example dapp does not implement key rotation, which is strongly recommended in a production environment. Key rotation involves periodically changing encryption keys and re-encrypting data to enhance security. In a production dapp, key rotation would be useful to limit the impact of a potential key compromise, or to limit access when users are added to or removed from sharing.

## Additional resources

- **[Password Manager with Metadata](../password_manager_with_metadata)** — if you need to store additional metadata alongside passwords.
- **[What are VetKeys](https://docs.internetcomputer.org/concepts/vetkeys)** — more information about VetKeys and VetKD.
- [Security best practices](https://docs.internetcomputer.org/guides/security/overview)
