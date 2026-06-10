# Daily Planner

Daily Planner is a full-stack ICP example featuring a monthly calendar that tracks daily notes and tasks stored on the network. For each day, a historic fact can be fetched from an external API using HTTPS outcalls, demonstrating how ICP canisters can access data from external services.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/daily_planner
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

To run the frontend in development mode with hot reload:

```bash
npm run dev
```

## Updating the Candid interface

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
