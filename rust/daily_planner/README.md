# Daily Planner

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/tree/master/rust/daily_planner)

> 🥷 **Try it live — no local setup.** [ICP Ninja](https://icp.ninja) is a web-based IDE that builds and deploys this project to the mainnet for free, right in your browser. Click the badge above, or hit **Deploy** if you're already in Ninja. To build and run it locally instead, follow the steps below.

Daily Planner is a full-stack ICP example featuring a monthly calendar that tracks daily notes and tasks stored on the network. For each day, a historic fact can be fetched from an external API using HTTPS outcalls, demonstrating how ICP canisters can access data from external services.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/daily_planner
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

To run the frontend in development mode with hot reload:

```bash
npm run dev
```

## Updating the Candid interface

```bash
icp build backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > backend/backend.did
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
