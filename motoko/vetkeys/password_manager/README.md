# VetKey Password Manager (Motoko)

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

This example uses `test_key_1` by default. To use a different [available master key](https://docs.internetcomputer.org/concepts/vetkeys/#api-overview), change the `VETKD_KEY_NAME` environment variable in `icp.yaml` before the first deploy.

The key name is read once at the first install and captured in stable state: it feeds vetKD key derivation, so a different key cannot decrypt what the old one encrypted — and since the canister only ever sees ciphertext, it cannot re-encrypt either. Changing the variable on a later upgrade is therefore silently ignored; only `icp deploy --mode reinstall`, which drops all data, switches keys. Re-keying live data would need application-level key rotation, which these examples do not implement.

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

> **Note.** The whole Encrypted Maps endpoint set comes from the `EncryptedMapsCanister` mixin (`mo:ic-vetkeys/encrypted_maps/Canister`), so the backend is a few lines instead of ~200 lines of hand-written delegation, and the exposed Candid matches what the `@icp-sdk/vetkeys` client expects by construction. Those methods are snake_case (rather than the usual Motoko camelCase) because the client calls the canister by these exact names. The mixin holds no stable state of its own: this actor declares the `EncryptedMapsState` and passes it in, so the persistent state stays a plain, visible stable variable the canister owns and can migrate. If you need to keep state linked to each value, use the `EncryptedMapsControlPlaneCanister` mixin instead — see the [`password_manager_with_metadata`](../password_manager_with_metadata/) example.

### Frontend (`frontend/`)

A **Svelte** application providing a user-friendly interface for managing vaults and passwords. It talks to the backend through the `@icp-sdk/vetkeys` Encrypted Maps client.

## Limitations

This example app does not implement key rotation, which is strongly recommended in a production environment. Key rotation involves periodically changing encryption keys and re-encrypting data to enhance security. In a production app, key rotation would be useful to limit the impact of a potential key compromise, or to limit access when users are added to or removed from sharing.

## Additional resources

- **[Password Manager with Metadata](../password_manager_with_metadata)** — if you need to store additional metadata alongside passwords.
- **[What are VetKeys](https://docs.internetcomputer.org/concepts/vetkeys)** — more information about VetKeys and VetKD.
- [Security best practices](https://docs.internetcomputer.org/guides/security/overview/)
