# Hello, world!

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/hello_world)

## Overview

This example demonstrates a simple "Hello, world!" application for ICP with both a Motoko backend canister and a frontend UI.

The backend canister stores a customizable greeting prefix (default: "Hello, ") as a stable variable, and exposes two methods:

- `setGreeting(prefix)` — updates the greeting prefix (persisted across canister upgrades).
- `greet(name)` — returns the greeting combined with the given name (e.g., "Hello, World!").

The frontend provides a simple form where users can enter their name and receive a personalized greeting from the backend canister.

This application's logic is written in [Motoko](https://docs.internetcomputer.org/motoko/home), a programming language designed specifically for developing canisters on ICP.

## Project structure

The `/backend` folder contains the Motoko canister, `app.mo`. The `/frontend` folder contains web assets for the application's user interface. The user interface is written with plain JavaScript, but any frontend framework can be used.

Edit the `mops.toml` file to add [Motoko dependencies](https://mops.one/) to the project.

## Build and deploy from the command line

### Prerequisites

- Install [Node.js](https://nodejs.org/en/download/)
- Install [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/hello_world
```

### Deployment

Start the local network and deploy:

```bash
icp network start -d
icp deploy
```

The frontend is served by the asset canister. To run the Vite dev server with hot reload during frontend development:

```bash
npm run dev
```

When done, stop the local network to free the port and clear state:

```bash
icp network stop
```

## Updating the Candid interface

The `backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, regenerate the `.did` file using the Motoko compiler:

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```
## Troubleshooting

### `icp network start` exits with status 101

The error message is:
Error: network launcher ... exited prematurely with status exit status: 101

The real cause is hidden in the log file at:
.icp/cache/networks/local/network-launcher/stderr.log

Always check that file first:
```bash
cat .icp/cache/networks/local/network-launcher/stderr.log
```
#### Cause: port 8000 already in use

The most common cause is another process (e.g. a Docker container) already bound to port 8000:
Failed to bind to address 127.0.0.1:8000: Address already in use (os error 98)

**Fix:**
```bash
# Find what's using port 8000
sudo lsof -i :8000
# or
sudo ss -tlnp | grep 8000

Then kill it:
bash# Replace <PID> with the number from the output above
sudo kill -9 <PID>

Or do it in one shot:
bashsudo fuser -k 8000/tcp

# Then retry
icp network start -d
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
