# HTTP counter

## Overview

The example demonstrates a counter dapp and an HTTP interface. It is essentially an iteration on the [counter canister](../Counter/README.md) which adds native HTTP interfaces.

This sample dapp provides an interface that exposes the following methods:

*  `http_request`, which can:
    * `GET` some static `gzip`ed data if `gzip` is accepted.
    * `GET` the counter otherwise.
    * Refer `POST`s to call `http_request_update`.
    * Returns `400` all other requests.
* `http_request_update`, which can:
    * `POST` to increment the counter.
        * returning some static `gzip`ed data if `gzip` is accepted.
        * otherwise returning the new counter value.
    * Returns `400` all other requests.


## Prerequisites 

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```
cd examples/motoko/http_counter
dfx start --background
```

### Step 2: Deploy the canister:

```
dfx deploy
```

### Step 3: Take note of canister ID to form URLs at which the `http_counter` is accessible.

```bash
CANISTER_ID=$(dfx canister id http_counter)

echo "http://localhost:4943/?canisterId=$CANISTER_ID"

echo "http://$CANISTER_ID.localhost:4943/"
```

### Step 4: All functionality of the canister can be exercised with the following commands:

```bash
CANISTER_ID=$(dfx canister id http_counter)

# Get the counter
curl "$CANISTER_ID.localhost:4943/" --resolve "$CANISTER_ID.localhost:4943:127.0.0.1"

# Get the static gziped query content
curl --compressed "$CANISTER_ID.localhost:4943/" --resolve "$CANISTER_ID.localhost:4943:127.0.0.1"

# Increment the counter
curl -X POST "$CANISTER_ID.localhost:4943/" --resolve "$CANISTER_ID.localhost:4943:127.0.0.1"

# Increment the counter and get the static gziped update content
curl --compressed -X POST "$CANISTER_ID.localhost:4943/" --resolve "$CANISTER_ID.localhost:4943:127.0.0.1"
```


## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspect is particularly relevant for this app:
* [Use HTTP asset certification and avoid serving your dApp through raw.ic0.app](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-http-asset-certification-and-avoid-serving-your-dapp-through-rawic0app), in case the HTTP responses should come with authenticity guarantees.  