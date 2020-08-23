## Makefile Example

[![Build Status](https://travis-ci.org/dfinity-lab/examples.svg?branch=master)](https://travis-ci.org/dfinity-lab/examples?branch=master)

Shows how to use a Makefile to encode dependencies between `dfx build`, `dfx canister create`, `dfx canister install`, and `dfx canister call`.

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
make call method=say message
```

Observe the following output.

```
dfx build
Building canisters...
touch .dfx/built
dfx canister install makefile --mode="reinstall"
Installing code for canister makefile, with canister_id 75hes-oqbaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-q
touch .dfx/installed/makefile
dfx canister call makefile say "message"
("message v7")
```

Run the same command again. It will have cached quite a bit, and the output will come much quicker:

```
▶ make call method=say message

dfx canister call makefile say "message"
("message v7")
```

Make a change to [src/main.mo](src/main.mo), e.g. increment the version number from 'v7' to 'v8'. Then re-run the command.

```
▶ make call method=say message

/Library/Developer/CommandLineTools/usr/bin/make build
dfx build
Building canisters...
touch .dfx/built
dfx canister install makefile --mode="reinstall"
Installing code for canister makefile, with canister_id 75hes-oqbaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-q
touch .dfx/installed/makefile
dfx canister call makefile say "message"
("message v8")
```

The source code was rebuilt and run, and the new 'v8' substring is included in the stdout.

```
▶ make call method=say message

dfx canister call makefile say "message"
("message v8")
```
