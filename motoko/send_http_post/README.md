# HTTP: POST

This example demonstrates how to use the ICP HTTPS outcalls feature to make a `POST` request from a Motoko canister. It sends a plain-text body to `postman-echo.com/post`, which echoes back the request as JSON, allowing you to verify the POST body and headers were sent correctly.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/send_http_post
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
