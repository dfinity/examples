# Canister snapshot download and upload

This example demonstrates the process of downloading and uploading canister snapshots. It features a canister called `quotes` which has some faulty data in its stable memory. You may think of it as data corrupted during a data migration or something similar. For the purposes of this example, it's simply a quote with a typo. 

The steps in this readme can be run all in sequence by invoking the `run.sh` script in bash. 

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install). Note: the Canister Snapshots feature requires `dfx` version `0.23.0-beta.3` or later.
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

## Step 1: Setup the project environment

Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the commands:

```bash
cd examples/rust/canister-snapshot-download
dfx start --clean --background
```

## Step 2: Compile and deploy `quotes` canister

```bash
dfx deploy
```

Also setup its initial state:

```bash
dfx canister call quotes "setup"
```

And check what the `print` endpoint returns: 

```bash
dfx canister call quotes "print"
```

Output: 
```bash
Colourless green ideas sleep furiously.
```

Clearly, this British spelling is not correct, as the author of the quote is American. Let's fix it by downloading the canister state, fixing the stable memory and loading the fixed version. 

## Step 3: Create and download snapshot

```bash
dfx canister stop quotes
dfx canister snapshot create quotes
```

This returns a snapshot id, similar to 
```bash
0000000000000000ffffffffff9000010101
```

Create a local directory and download the new snapshot: 
```bash
mkdir ./snapshots
dfx canister snapshot download --dir ./snapshots quotes 0000000000000000ffffffffff9000010101
```

## Step 4: Manipulate and upload the snapshot
View the file in ./snapshot/stable_memory.bin. This is a binary file, but the first few bytes are ASCII characters. 

Fix the spelling by running:
```bash
sed -i -e 's/Colour/Color/g' ./snapshots/stable_memory.bin
```

Upload the fixed snapshot.
```bash
dfx canister snapshot upload --dir ./snapshots quotes
```

This will return a new snapshot id, similar to
```bash
0000000000000001ffffffffff9000010101
```

## Step 5: Load the snapshot and verify

```bash
dfx canister snapshot load quotes 0000000000000001ffffffffff9000010101
dfx canister start quotes
dfx canister call quotes "print"
```

Output: 
```bash
Colorless green ideas sleep furiously.
```

Clean up.
```bash
rm -rf ./snapshots
dfx stop
```

## Conclusion

The ability to download and upload snapshots enables various new use cases: 
- Keeping canister backups on disk rather than on-chain.
- Cloning canister state into another canister.
- Migrating canister state to a different subnet. 
- Fixing faulty state or performing data migrations that would be prohibitive on-chain.