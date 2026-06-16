# FileVault

FileVault is a file storage application that allows you to upload files from your local computer and store them onchain. FileVault uses Internet Identity (II) for user login and authentication. Once files are uploaded, they can be downloaded at a later time, or they can be deleted. Each user's files are stored under their Internet Identity principal, so files are private to each authenticated user.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/filevault
```

### Deploy and test

Start the local network, deploy, and run tests:

```bash
icp network start -d
icp deploy
make test
icp network stop
```

The frontend is served by the asset canister. To run the Vite dev server with hot reload during frontend development:

```bash
npm run dev
```

## Updating the Candid interface

The `backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, regenerate the `.did` file using the Motoko compiler:

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
