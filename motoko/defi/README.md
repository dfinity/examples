# Defi Example

This repo contains a simple defi exchange that demonstrates the interaction with ICP and tokens on the IC. For a more detailed explanation check out the [architecture.md](architecture.md) file or visit the official [documentation](https://smartcontracts.org/docs/examples/defi.html)

This example can be seen running, here:
- frontend: https://gzz56-daaaa-aaaal-qai2a-cai.ic0.app/
- AkitaDIP20: `gl7kh-pqaaa-aaaal-qaiza-cai`
- GoldenDIP20: `gm6mt-ciaaa-aaaal-qaizq-cai`

## Dependencies

- [dfx](https://smartcontracts.org/docs/developers-guide/install-upgrade-remove.html)
- [cmake](https://cmake.org/)
- [npm](https://nodejs.org/en/download/)

If you want to deploy the rust version, make sure you add wasm as a target:

```
rustup target add wasm32-unknown-unknown
```
## Quickstart

This deploys a local ledger, two DIP20 Tokens, II, and our project.

```bash
git clone --recurse-submodules --shallow-submodules https://github.com/dfinity/examples.git
# for the rust imeplementation examples/rust/defi
cd examples/motoko/defi
make install
```

The install scripts output the URL to visit the exchange frontend, or you can regenerate the URL `"http://localhost:8000?canisterId=$(dfx canister id frontend)"`. To interact with the exchange, you can create a local internet identity by clicking the login button. 

You can give yourself some tokens and ICP by running an initialization script with your II Principal that you can copy from the frontend. After adding balanced reload the frontend.

```bash
make init-local II_PRINCIPAL=<YOUR II PRINCIPAL>
```

To trade with yourself, you can open a second incognito browser window. 

## Development

We deploy the canisters to a *system* subnet by specifying
```
    "replica": {
      "subnet_type": "system"
    }
```
in the configuration file `dfx.json` because the size of the Wasm file for the ledger canister exceeds the limit of 2MiB for the default *application* subnet.

Reinstall backend canister

```bash
dfx deploy defi_dapp -m reinstall --argument '(null)'
```

Local frontend development

```bash
make frontend
```

## Test

Run from home directory

```bash
make test
```


## Examples

### Demo

See [demo.sh](test/demo.sh).

### Trade

See [trade.sh](test/trade.sh).

### Token transfers

See [transfer.sh](test/transfer.sh).

### Deploy DIP20 token

See [deploy_dip20.sh](scripts/deploy_dip20.sh).

## Roadmap
- Improve Plug wallet support
- Make exchange more predictable

## Troubleshooting

### DFX deploys canisters with the same ID

Clear `.dfx` directories

```
make clean
```

### Deposit address not loading 

Make sure you are logged out with II and refresh the page.

### Address already in use

This is probably due to an orphan `dfx` instance. Find the PID of the orphan dfx instance

```
ps -xa | grep dfx
kill <dfx PID>
```

### Missing cmake

```
   Compiling tempfile v3.3.0
   Compiling quote v1.0.14
error: failed to run custom build command for `wabt-sys v0.8.0`

Caused by:
  process didn't exit successfully: `/var/folders/81/cvnmgym54z15l8469p4k0yc40000gn/T/cargo-installQ7mfnX/release/build/wabt-sys-8ee9fea2b803bc94/build-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-env-changed=WABT_CXXSTDLIB
  cargo:rerun-if-env-changed=CXXSTDLIB
  CMAKE_TOOLCHAIN_FILE_aarch64-apple-darwin = None
  CMAKE_TOOLCHAIN_FILE_aarch64_apple_darwin = None
  HOST_CMAKE_TOOLCHAIN_FILE = None
  CMAKE_TOOLCHAIN_FILE = None
  CMAKE_GENERATOR_aarch64-apple-darwin = None
  CMAKE_GENERATOR_aarch64_apple_darwin = None
  HOST_CMAKE_GENERATOR = None
  CMAKE_GENERATOR = None
  CMAKE_PREFIX_PATH_aarch64-apple-darwin = None
  CMAKE_PREFIX_PATH_aarch64_apple_darwin = None
  HOST_CMAKE_PREFIX_PATH = None
  CMAKE_PREFIX_PATH = None
  CMAKE_aarch64-apple-darwin = None
  CMAKE_aarch64_apple_darwin = None
  HOST_CMAKE = None
  CMAKE = None
  running: "cmake" "/Users/timgretler/.cargo/registry/src/github.com-1ecc6299db9ec823/wabt-sys-0.8.0/wabt" "-DBUILD_TESTS=OFF" "-DBUILD_TOOLS=OFF" "-DCMAKE_INSTALL_PREFIX=/var/folders/81/cvnmgym54z15l8469p4k0yc40000gn/T/cargo-installQ7mfnX/release/build/wabt-sys-f412d7d66c1e351f/out" "-DCMAKE_C_FLAGS= -ffunction-sections -fdata-sections -fPIC -arch arm64" "-DCMAKE_C_COMPILER=/usr/bin/cc" "-DCMAKE_CXX_FLAGS= -ffunction-sections -fdata-sections -fPIC -arch arm64" "-DCMAKE_CXX_COMPILER=/usr/bin/c++" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -arch arm64" "-DCMAKE_ASM_COMPILER=/usr/bin/cc" "-DCMAKE_BUILD_TYPE=Release"

  --- stderr
  thread 'main' panicked at '
  failed to execute command: No such file or directory (os error 2)
  is `cmake` not installed?

  build script failed, must exit now', /Users/timgretler/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.48/src/lib.rs:975:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to compile `ic-cdk-optimizer v0.3.1`, intermediate artifacts can be found at `/var/folders/81/cvnmgym54z15l8469p4k0yc40000gn/T/cargo-installQ7mfnX`

Caused by:
  build failed

```
Need to install cmake in your environment

MacOS: `brew install cmake`
Debian/Ubuntu: `apt install cmake`

### Compiling takes ages

Check for a cycle in the dependencies.

### Incorrect result from `bundle.js`

```
git clone https://github.com/dfinity/examples.git
cd examples
git checkout defi-example
git submodule update --init --recursive
cd motoko/defi
make install
```

