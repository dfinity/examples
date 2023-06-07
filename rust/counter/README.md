# Counter

## Rust variant

### Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](../developer-docs/setup/install/index.mdx).
- [x] Download the following project files from GitHub: https://github.com/dfinity/examples/

Begin by opening a terminal window.

 ### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```
cd examples/rust/counter
dfx start --background
```

 ### Step 2: Test the canister:

```
cargo test
```

 ### Step 3: Deploy the canister:

```
dfx deploy
```

 ### Step 4: Set the value of the counter:

```
dfx canister call counter set '(7)'
```

 ### Step 5: Increment the value of the counter:

```
dfx canister call counter inc
```

 ### Step 6: Get the value of the counter:

```
dfx canister call counter get
```

The following output should be returned:

```
(8 : nat)
```


## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

