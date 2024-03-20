---
keywords: [beginner, rust, counter]
---

# Counter

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/counter)

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).
- [x] Download the following project files from GitHub: `git clone https://github.com/dfinity/examples/`

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```bash
cd examples/rust/counter
dfx start --background
```

### Step 2: Test the canister:

```bash
cargo test
```

### Step 3: Deploy the canister:

```bash
dfx deploy
```

### Step 4: Set the value of the counter:

```bash
dfx canister call counter set '(7)'
```

### Step 5: Increment the value of the counter:

```bash
dfx canister call counter inc
```

### Step 6: Get the value of the counter:

```bash
dfx canister call counter get
```

The following output should be returned:

```bash
(8 : nat)
```


## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

