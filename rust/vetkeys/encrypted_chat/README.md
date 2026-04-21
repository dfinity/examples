# Encrypted Chat

| Rust backend | [![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/vetkeys/encrypted_chat/rust) |
| --- | --- |

> **Disclaimer**: This is an *unfinished* prototype. DO NOT USE IN PRODUCTION.

The **Encrypted Chat** example demonstrates how to use **[vetKeys](https://internetcomputer.org/docs/building-apps/network-features/vetkeys/introduction)** to build an end-to-end encrypted messaging application on the **Internet Computer (IC)**. Messages are encrypted on the sender's device and can only be decrypted by the intended recipients — the backend canister never sees plaintext.

See [SPEC.md](./SPEC.md) for the full technical specification.

## Features

- **End-to-end encrypted messaging**: Messages are encrypted client-side using keys derived from vetKeys.
- **Direct and group chats**: Support for both one-on-one and multi-participant conversations.
- **Symmetric ratchet key rotation**: Keys evolve over time via a symmetric ratchet, providing forward security.
- **Disappearing messages**: Messages expire and are automatically purged from both frontend and backend.
- **Encrypted state recovery**: Users can recover their decryption capability across devices using IBE-encrypted key resharing.

## Setup

### Prerequisites

- [Internet Computer software development kit](https://internetcomputer.org/docs/building-apps/getting-started/install)
- [npm](https://www.npmjs.com/package/npm)

### Deploy the Canisters Locally

From the `rust` folder, run:
```bash
dfx start --background && dfx deploy
```

## Example Components

### Backend

The Rust backend canister manages:
- Chat creation (direct and group) with configurable key rotation and message expiration periods.
- Encrypted message storage and retrieval.
- VetKey derivation for chat encryption keys, with epoch-based rotation.
- Group membership management (add/remove participants).

### Frontend

The frontend is a SvelteKit application providing:
- Internet Identity authentication.
- Real-time encrypted messaging interface.
- Local message caching with IndexedDB.
- Automatic key ratcheting and vetKey epoch management.

To run the frontend in development mode with hot reloading (after running `dfx deploy`):

```bash
cd frontend
npm run dev
```

## Additional Resources

- **[SPEC.md](./SPEC.md)** - Full technical specification of the encryption protocol.
- **[What are VetKeys](https://internetcomputer.org/docs/building-apps/network-features/encryption/vetkeys)** - For more information about VetKeys and VetKD.
