# Counter canister in WebAssembly

This example demonstrates a counter application written directly in WebAssembly Text Format (WAT). The compiled Wasm module is only 389 bytes.

It can be a good way to learn and experiment with the [IC System API](https://docs.internetcomputer.org/references/ic-interface-spec/canister-interface/#system-api).

## Prerequisites

- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [WABT](https://github.com/WebAssembly/wabt) (for `wat2wasm`): `brew install wabt`

## Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

## Manual testing

```bash
icp canister call --query counter get '()'
# (0 : int64)

icp canister call counter inc '()'
icp canister call --query counter get '()'
# (1 : int64)

icp canister call counter set '(42 : int64)'
icp canister call --query counter get '()'
# (42 : int64)
```
