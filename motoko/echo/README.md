![Compatibility](https://img.shields.io/badge/compatibility-0.6.10-blue)
[![Build Status](https://travis-ci.org/dfinity-lab/examples.svg?branch=master)](https://travis-ci.org/dfinity-lab/examples?branch=master)

# Echo

### Prerequisites

Before building the example application, verify the following:

* You have downloaded and installed the DFINITY Canister SDK as described in [Download and install](https://sdk.dfinity.org/docs/quickstart/quickstart.html#download-and-install).
* You have stopped any Internet Computer network processes running on the local computer.

### Demo

Start a local internet computer.

```bash
dfx start
```

Execute the following commands in another tab.

```bash
dfx canister create echo
dfx build
dfx canister install echo
dfx canister call echo say '("This is a test.")'
```

Observe the following result.

```
("This is a test.")
```
