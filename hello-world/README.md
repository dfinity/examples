## Hello World

### Prerequisites

- [DFINITY SDK](https://sdk.dfinity.org)

### Demo

Start a local internet computer.

```bash
dfx start
```

Execute the following commands in another tab.

```bash
dfx build
dfx canister install --all
dfx canister call hello-world main
```

Observe the internet computer console.
```
debug.print: Hello World!
```
