## Echo

[![Build Status](https://travis-ci.org/dfinity-lab/examples.svg?branch=master)](https://travis-ci.org/dfinity-lab/examples?branch=master)

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
dfx canister call echo say '("This is a test.")'
```

Observe the following result.

```
("This is a test.")
```
