# VetKey Password Manager

<!-- TODO: re-enable once icp.ninja supports icp-cli (currently requires dfx)
| Motoko backend | [![](https://icp.ninja/assets/open.svg)](http://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/vetkeys/password_manager/motoko)|
| --- | --- |
| Rust backend | [![](https://icp.ninja/assets/open.svg)](http://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/vetkeys/password_manager/rust) |
-->

The **VetKey Password Manager** is an example application demonstrating how to use **VetKeys** and **Encrypted Maps** to build a secure, decentralized password manager on the **Internet Computer (IC)**. This application allows users to create password vaults, store encrypted passwords, and share vaults with other users via their **Internet Identity Principal**.

## Features

- **Secure Password Storage**: Uses VetKey to encrypt passwords before storing them in Encrypted Maps.
- **Vault-Based Organization**: Users can create multiple vaults, each containing multiple passwords.
- **Access Control**: Vaults can be shared with other users via their **Internet Identity Principal**.

## Setup

### Prerequisites

- [ICP CLI](https://cli.internetcomputer.org)
- [npm](https://www.npmjs.com/package/npm)

### (Optionally) Choose a Different Master Key

This example uses `test_key_1` by default. To use a different [available master key](https://docs.internetcomputer.org/concepts/vetkeys/#api-overview), change the `init_args` value in `icp.yaml` to the desired key before running `icp deploy` in the next step.

### Folder Structure

This example provides both a **Rust** and a **Motoko** backend, sharing a common `frontend/`:

```
password_manager/
├── frontend/       ← shared frontend (symlinked into rust/ and motoko/)
├── motoko/backend/   ← Motoko backend + icp.yaml
└── rust/           ← Rust backend + icp.yaml
```

### Deploy the Canisters Locally

Deploy with the **Motoko** backend:
```bash
cd motoko/backend
icp network start -d && icp deploy
```

Or deploy with the **Rust** backend:
```bash
cd rust
icp network start -d && icp deploy
```

To run the frontend in development mode with hot reloading (after running `icp deploy`):
```bash
cd frontend
npm run dev:motoko   # if you deployed the Motoko backend
# or
npm run dev:rust     # if you deployed the Rust backend
```

When you are done testing, stop the local network to free up resources and unblock the default port for other projects:
```bash
icp network stop
```

## Example Components

### Backend

The backend consists of an **Encrypted Maps**-enabled canister that securely stores passwords.

### Frontend

The frontend is a **Svelte** application providing a user-friendly interface for managing vaults and passwords.

## Limitations

This example dapp does not implement key rotation, which is strongly recommended in a production environment.
Key rotation involves periodically changing encryption keys and re-encrypting data to enhance security.
In a production dapp, key rotation would be useful to limit the impact of potential key compromise if a malicious party gains access to a key, or to limit access when users are added or removed from note sharing.

## Additional Resources

- **[Password Manager with Metadata](../password_manager_with_metadata/)** - If you need to store additional metadata alongside passwords.
