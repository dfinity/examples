# Flying Ninja

Flying Ninja is a 2D side-scroller game where players control a ninja character using the space bar to move up and down, dodging obstacles to earn points. When the game ends, players can submit their score to an on-chain leaderboard backed by a Rust canister on ICP.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/flying_ninja
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

For frontend development with hot reload:

```bash
npm run dev --prefix frontend
```

## Updating the Candid interface

The `backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, rebuild the canister and regenerate the `.did` file:

```bash
icp build backend
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > backend/backend.did
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
