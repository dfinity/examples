# VetKey Password Manager with Metadata (Rust)

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
- Install the [Rust toolchain](https://www.rust-lang.org/tools/install), then add the WASM target: `rustup target add wasm32-unknown-unknown`

### (Optionally) choose a different master key

This example uses `test_key_1` by default. To use a different [available master key](https://docs.internetcomputer.org/concepts/vetkeys/#api-overview), change the `init_args` value in `icp.yaml` before deploying.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/vetkeys/password_manager_with_metadata
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

An **Encrypted Maps**-enabled Rust canister that stores encrypted passwords together with unencrypted metadata (URLs, tags) in atomic update calls.

> **Note.** A plain Encrypted Maps canister is generated in one line by the `ic_vetkeys::export_encrypted_maps_canister!(...)` macro — see the [`password_manager`](../password_manager/) example. This example uses the macro's `custom_value_endpoints` form instead, because it maintains a custom invariant: every encrypted value has a matching metadata row. That form generates the stable state, the `#[init]`/`#[post_upgrade]` hooks and the control-plane endpoints (vetKD keys, access control, map-name enumeration), but **none** of the endpoints that read or write encrypted values — so the plain `insert_encrypted_value`/`remove_encrypted_value` mutators, which would write a value with no metadata row and desync the two stores, are never exposed. The canister writes its own `*_with_metadata` endpoints on top of the `with_encrypted_maps`/`with_encrypted_maps_mut` accessors the macro emits, reusing the library's crypto and access-control logic. Reach for the plain form when the standard Encrypted Maps interface is enough, and for `custom_value_endpoints` when you layer your own state on each value.

### Frontend (`frontend/`)

A **Svelte** application for managing vaults and passwords. It uses the `@icp-sdk/vetkeys` Encrypted Maps client for the crypto operations and a canister actor (bindings generated from `backend/backend.did` by the `@icp-sdk/bindgen` Vite plugin) for the metadata methods.

#### Derived-key caching

`EncryptedMaps` derives and caches the vetKey material used to encrypt/decrypt values. As of `@icp-sdk/vetkeys` 0.5.0 this cache is **in memory by default** (discarded on page reload). This example opts into cross-reload persistence with `IndexedDbDerivedKeyMaterialCache`, giving the store a **per-identity namespace** (`vetkeys-<principal>`) so one identity's keys are never served to another on the same origin, and it calls `clearCache()` on the `EncryptedMaps` instance on logout to drop the cached material.

**Verifying `clearCache()` on logout.** `clearCache()` *empties* the cache's object store; it does not delete the database, so an empty `vetkeys-<principal>` database remaining in the list is expected (an empty store holds no usable key material). To check it:

1. Open DevTools → **Application → IndexedDB**.
2. Expand `vetkeys-<principal>` → `derived-key-material`. While logged in and after opening a vault it holds one or more entries; after logout it should hold **0**.
3. DevTools caches this view — click **Refresh database** after logging out, otherwise stale entries appear to linger.

You will also see an unrelated `icp-sdk-<host>` database (object store `subnetNodeKeys`): that is the agent's cache of public subnet node keys, not key material, and is safe to ignore.

## Updating the Candid interface

`backend/backend.did` defines the backend's public interface; the frontend bindings are generated from it during the build. If you change the backend's public API, regenerate it:

```bash
icp build backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > backend/backend.did
```

## Limitations

This example app does not implement key rotation, which is strongly recommended in a production environment. Key rotation involves periodically changing encryption keys and re-encrypting data to enhance security. In a production app, key rotation would be useful to limit the impact of a potential key compromise, or to limit access when users are added to or removed from sharing.

## Additional resources

- **[Basic Password Manager](../password_manager)** — a simpler example without metadata.
- **[What are VetKeys](https://docs.internetcomputer.org/concepts/vetkeys)** — more information about VetKeys and VetKD.
- [Security best practices](https://docs.internetcomputer.org/guides/security/overview/)
