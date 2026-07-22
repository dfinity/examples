# VetKey Password Manager (Motoko)

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/vetkeys/password_manager)

Also available in: [Rust](../../../rust/vetkeys/password_manager)

The **VetKey Password Manager** is an example application demonstrating how to use **VetKeys** and **Encrypted Maps** to build a secure, decentralized password manager on the **Internet Computer (IC)**. This application allows users to create password vaults, store encrypted passwords, and share vaults with other users via their **Internet Identity Principal**.

## Features

- **Secure Password Storage**: Uses VetKey to encrypt passwords before storing them in Encrypted Maps.
- **Vault-Based Organization**: Users can create multiple vaults, each containing multiple passwords.
- **Access Control**: Vaults can be shared with other users via their **Internet Identity Principal**.

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
cd examples/motoko/vetkeys/password_manager
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

An **Encrypted Maps**-enabled Motoko canister that securely stores passwords.

> **Note on naming.** The backend methods are snake_case (rather than the usual Motoko camelCase) because the `@icp-sdk/vetkeys` Encrypted Maps client calls the canister by these exact names — renaming them would break the frontend. The delegation methods are hand-written for now; an upstream Motoko actor mixin that generates this endpoint set automatically is in progress ([dfinity/vetkeys#405](https://github.com/dfinity/vetkeys/pull/405)).

### Frontend (`frontend/`)

A **Svelte** application providing a user-friendly interface for managing vaults and passwords. It talks to the backend through the `@icp-sdk/vetkeys` Encrypted Maps client.

## Limitations

This example app does not implement key rotation, which is strongly recommended in a production environment. Key rotation involves periodically changing encryption keys and re-encrypting data to enhance security. In a production app, key rotation would be useful to limit the impact of a potential key compromise, or to limit access when users are added to or removed from sharing.

## Additional resources

- **[Password Manager with Metadata](../../../rust/vetkeys/password_manager_with_metadata)** — if you need to store additional metadata alongside passwords.
- **[What are VetKeys](https://docs.internetcomputer.org/concepts/vetkeys)** — more information about VetKeys and VetKD.
- [Security best practices](https://docs.internetcomputer.org/guides/security/overview/)
