# Encrypted notes: vetKD

<!--
ICP Ninja links removed: icp.ninja support requires dfx which is no longer used in this example.
| Motoko backend | [![](https://icp.ninja/assets/open.svg)](http://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/vetkeys/encrypted_notes_dapp_vetkd/motoko)|
| --- | --- |
| Rust backend | [![](https://icp.ninja/assets/open.svg)](http://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/vetkeys/encrypted_notes_dapp_vetkd/rust) |
-->

Encrypted notes is an example dapp for authoring and storing confidential information on the Internet Computer (ICP) in the form of short pieces of text. Users can create and access their notes via any number of automatically synchronized devices authenticated via Internet Identity (II). Notes are stored confidentially using vetKeys. The end-to-end encryption is performed by the dapp's frontend.

In particular, the notes are encrypted with an AES key that is derived (directly in the browser) from a note-ID-specific vetKey obtained from the backend canister (in encrypted form, using an ephemeral transport key), which itself obtains it from the vetKD system API. This way, there is no need for any device management in the dapp, plus sharing of notes becomes possible.

The vetKey used to encrypt and decrypt a note is note-ID-specific (and not, for example, principal-specific) to enable the sharing of notes between users. The derived AES keys are stored as non-extractable CryptoKeys in an IndexedDB in the browser for efficiency so that their respective vetKey only has to be fetched from the server once.

## Prerequisites

- [x] Install the [ICP CLI](https://cli.internetcomputer.org).
- [x] Install [npm](https://www.npmjs.com/package/npm).

## Deploy the Canisters Locally

To deploy with the Motoko backend, run from the `motoko` folder:
```bash
icp network start -d && icp deploy
```

To deploy with the Rust backend instead, run the same command from the `rust` folder.

## Running the Frontend in Development Mode

After deploying, run from the `frontend` folder:
```bash
# For Motoko backend:
npm run dev:motoko

# For Rust backend:
npm run dev:rust
```

## Example Components

### Backend

The backend consists of a canister that stores encrypted notes. It is automatically deployed with `icp deploy`.

### Frontend

The frontend is a **Svelte** application providing a user-friendly interface for managing encrypted notes.

## Limitations

This example dapp does not implement key rotation, which is strongly recommended in a production environment.

## Troubleshooting

If you run into issues, clearing all the application-specific IndexedDBs in the browser might help. For example in Chrome, go to Inspect → Application → Local Storage → Clear All, and then reload.
