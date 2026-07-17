# VetKey Password Manager with Metadata (Motoko)

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/vetkeys/password_manager_with_metadata)

Also available in: [Rust](../../../rust/vetkeys/password_manager_with_metadata)

The **VetKey Password Manager** is an example application demonstrating how to use **VetKeys** and **Encrypted Maps** to build a secure, decentralized password manager on the **Internet Computer (IC)**. This application allows users to create password vaults, store encrypted passwords, and share vaults with other users via their **Internet Identity Principal**.

This version extends the basic password manager by supporting unencrypted metadata, such as URLs and tags, alongside encrypted passwords. The goal is to demonstrate how to make atomic updates to the Encrypted Maps canister, storing both encrypted and unencrypted data in a single update call.

## Features

- **Secure Password Storage**: Uses VetKey to encrypt passwords before storing them in Encrypted Maps.
- **Vault-Based Organization**: Users can create multiple vaults, each containing multiple passwords.
- **Access Control**: Vaults can be shared with other users via their **Internet Identity Principal**.
- **Atomic Updates**: Stores encrypted passwords along with unencrypted metadata in a single update call.

## Build and deploy from the command line

### Prerequisites

- Install [Node.js](https://nodejs.org/en/download/)
- Install [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- Install [ic-mops](https://mops.one): `npm install -g ic-mops`

### (Optionally) choose a different master key

This example uses `test_key_1` by default. To use a different [available master key](https://docs.internetcomputer.org/concepts/vetkeys/#api-overview), change the `init_args` value in `icp.yaml` before deploying.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/vetkeys/password_manager_with_metadata
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

An **Encrypted Maps**-enabled Motoko canister that stores encrypted passwords together with unencrypted metadata (URLs, tags) in atomic update calls.

> **Note on naming.** The backend methods are snake_case (rather than the usual Motoko camelCase). The standard Encrypted Maps methods are called by these exact names by the `@icp-sdk/vetkeys` Encrypted Maps client, and the custom metadata methods follow the same convention — renaming them would break the frontend. An upstream Motoko actor mixin that generates the Encrypted Maps endpoint set automatically is in progress ([dfinity/vetkeys#405](https://github.com/dfinity/vetkeys/pull/405)).

### Frontend (`frontend/`)

A **Svelte** application for managing vaults and passwords. It uses the `@icp-sdk/vetkeys` Encrypted Maps client for the crypto operations and a canister actor (bindings generated from `backend/backend.did` by the `@icp-sdk/bindgen` Vite plugin) for the metadata methods.

## Limitations

This example app does not implement key rotation, which is strongly recommended in a production environment. Key rotation involves periodically changing encryption keys and re-encrypting data to enhance security. In a production app, key rotation would be useful to limit the impact of a potential key compromise, or to limit access when users are added to or removed from sharing.

## Additional resources

- **[Basic Password Manager](../../../rust/vetkeys/password_manager)** — a simpler example without metadata.
- **[What are VetKeys](https://docs.internetcomputer.org/concepts/vetkeys)** — more information about VetKeys and VetKD.
- [Security best practices](https://docs.internetcomputer.org/guides/security/overview/)
