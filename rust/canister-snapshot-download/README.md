# Canister snapshot download and upload

This example demonstrates the process of downloading and uploading canister snapshots.
It features a canister called `backend` which stores some data in stable memory.
The example walks through downloading a snapshot, manipulating the binary stable memory
off-chain, uploading the fixed snapshot, and restoring the canister from it.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/rust/canister-snapshot-download
```

### Deploy and test
```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

## How it works

The backend canister stores a quote in stable memory using the low-level stable memory API.
The initial quote contains a British spelling ("Colourless") that can be fixed off-chain:

1. **Setup**: call `setup` to write the initial data into stable memory.
2. **Snapshot**: stop the canister, create a snapshot, then start it again.
3. **Download**: `icp canister snapshot download --dir ./snapshots backend <snapshot-id>` saves
   `stable_memory.bin` and other files to the local directory.
4. **Manipulate**: edit the binary file directly (e.g. `sed -i 's/Colour/Color/g' ./snapshots/stable_memory.bin`).
5. **Upload**: `icp canister snapshot upload --dir ./snapshots backend` returns a new snapshot ID.
6. **Load**: stop the canister, load the new snapshot, then start it again.
7. **Verify**: call `print` to confirm the spelling fix is live.

This workflow enables several use cases:
- Keeping canister backups on disk rather than on-chain.
- Cloning canister state into another canister.
- Migrating canister state to a different subnet.
- Fixing faulty state or performing data migrations that would be prohibitive on-chain.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize
yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview)
for developing on the Internet Computer. This example may not implement all the best practices.
