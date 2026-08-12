# Encrypted Notes: vetKD (Motoko)

Encrypted notes is an example app for authoring and storing confidential information on the Internet Computer (ICP) in the form of short pieces of text. Users can create and access their notes via any number of automatically synchronized devices authenticated via Internet Identity (II). Notes are stored confidentially using vetKeys. The end-to-end encryption is performed by the app's frontend.

In particular, the notes are encrypted with an AES key that is derived (directly in the browser) from a note-ID-specific vetKey obtained from the backend canister (in encrypted form, using an ephemeral transport key), which itself obtains it from the vetKD system API. This way, there is no need for any device management in the app, plus sharing of notes becomes possible.

The vetKey used to encrypt and decrypt a note is note-ID-specific (and not, for example, principal-specific) to enable the sharing of notes between users. The derived AES keys are stored as non-extractable CryptoKeys in an IndexedDB in the browser for efficiency so that their respective vetKey only has to be fetched from the server once.

## Build and deploy from the command line

### Prerequisites

- Install [Node.js](https://nodejs.org/en/download/)
- Install [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- Install [ic-mops](https://mops.one): `npm install -g ic-mops`

### (Optionally) choose a different master key

This example uses `test_key_1` by default. To use a different [available master key](https://docs.internetcomputer.org/concepts/vetkeys/#api-overview), change the `VETKD_KEY_NAME` environment variable in `icp.yaml` before the first deploy.

Do not change it once the canister holds data: the key name feeds vetKD key derivation, so a different key cannot decrypt what the old one encrypted.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/vetkeys/encrypted_notes_app_vetkd
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

A single Motoko canister that stores encrypted notes. It is deployed automatically with `icp deploy`.

### Frontend (`frontend/`)

A **Svelte** application providing a user-friendly interface for managing encrypted notes. Canister bindings are generated from `backend/backend.did` at build time by the `@icp-sdk/bindgen` Vite plugin.

## Updating the Candid interface

`backend/backend.did` defines the backend's public interface; the frontend bindings are generated from it during the build. If you change the backend's public API, regenerate it:

```bash
mops generate candid backend
```

## Limitations

This example app does not implement key rotation, which is strongly recommended in a production environment.

## Troubleshooting

If you run into issues, clearing all the application-specific IndexedDBs in the browser might help. For example in Chrome, go to Inspect → Application → Local Storage → Clear All, and then reload.

## API level

This example intentionally uses the **raw vetKD management canister API** (`encryptedSymmetricKeyForNote`, `symmetricKeyVerificationKeyForNote`) to demonstrate how vetKD works at the protocol level.

For most applications, the higher-level [`EncryptedMaps`](https://github.com/dfinity/vetkeys/tree/main/frontend/ic_vetkeys/src/encrypted_maps) abstraction from `@icp-sdk/vetkeys` is the recommended approach — it handles key derivation, caching, and access control internally without requiring a custom crypto layer. See the **VetKD Password Manager** ([`../password_manager`](../password_manager)) and **Password Manager with Metadata** ([`../password_manager_with_metadata`](../password_manager_with_metadata)) examples for how `EncryptedMaps` is used in practice.

## Additional resources

- **[What are VetKeys](https://docs.internetcomputer.org/concepts/vetkeys)** — more information about VetKeys and VetKD.
- [Security checklist for this example](security-checklist.md)
- [Security best practices](https://docs.internetcomputer.org/guides/security/overview/)
