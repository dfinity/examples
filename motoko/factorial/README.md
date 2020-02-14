## Factorial

[![Build Status](https://travis-ci.org/dfinity-lab/examples.svg?branch=master)](https://travis-ci.org/dfinity-lab/examples?branch=master)

### Prerequisites

* You have downloaded and installed the SDK as described in [Getting started](https://sdk.dfinity.org/developer-guide/getting-started.html).

### Demo

Start a local internet computer.

```bash
dfx start
```

Execute the following commands in another tab.

```bash
dfx build
dfx canister install --all
dfx canister call factorial fac '(20)'
```

Observe the following result.

```
(2432902008176640000)
```
