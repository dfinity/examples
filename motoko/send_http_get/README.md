# HTTP: GET

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/send_http_get)

## Overview

This example demonstrates how to use the Internet Computer's HTTPS outcalls feature to make a `GET` request from a Motoko canister. It sends a request to `postman-echo.com` and returns the echoed JSON response, showing how to pass query parameters and headers through HTTP outcalls.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/send_http_get
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP dapp.
