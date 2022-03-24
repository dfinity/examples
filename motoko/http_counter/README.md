# Http Counter

The example demonstrates a counter application and an http interface. It is essentially an iteration on the [Counter canister](../Counter/README.md) which adds native HTTP interfaces.

# Introduction

The application provides an interface that exposes the following methods:

*  `http_request`, which can:
    * `GET` some static `gzip`ed data if `gzip` is accepted
    * `GET` the counter otherwise
    * Refer `POST`s to call `http_request_update`
    * Returns `400` all other requests
* `http_request_update`, which can:
    * `POST` to increment the counter
        * returning some static `gzip`ed data if `gzip` is accepted
        * otherwise returning the new counter value
    * Returns `400` all other requests

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Take note of canister id to form URLs at which the `http_counter` is accessible.

```bash
CANISTER_ID=$(dfx canister id http_counter)

echo "http://localhost:8000/?canisterId=$CANISTER_ID"

echo "http://$CANISTER_ID.localhost:8000/"
```

All functionality of the canister can be exercised with the following commands:

```bash
CANISTER_ID=$(dfx canister id http_counter)

# Get the counter
curl "$CANISTER_ID.localhost:8000/" --resolve "$CANISTER_ID.localhost:8000:127.0.0.1"

# Get the static gziped query content
curl --compressed "$CANISTER_ID.localhost:8000/" --resolve "$CANISTER_ID.localhost:8000:127.0.0.1"

# Increment the counter
curl -X POST "$CANISTER_ID.localhost:8000/" --resolve "$CANISTER_ID.localhost:8000:127.0.0.1"

# Increment the counter and get the static gziped update content
curl --compressed -X POST "$CANISTER_ID.localhost:8000/" --resolve "$CANISTER_ID.localhost:8000:127.0.0.1"
```
