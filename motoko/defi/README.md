# defi-test


## start local dev env

- Install dfx

```
dfx start
```

initialize submodule

```
git submodule update --init --recursive
```

## Quickstart

Setup local environment. This deploys a local ledger and two DIP20 Tokens 

```bash
sh install-local-ledger.sh 
```

## Development

Reinstall backend canister

```bash
dfx deploy defi-dapp -m reinstall
```

Local frontend development

```bash
cd src/defi-dapp-frontend/src/frontend
npm install
npm dev run
```


## Examples

### Token transfers

```bash
# deploy defi app
dfx deploy defi-dapp -m reinstall
# set allowance on DIP20 tokens
DEX_PRINCIPLE=$(dfx canister --no-wallet id defi-dapp)
dfx canister --no-wallet call AkitaDIP20 approve  '(principal '\"$DEX_PRINCIPLE\"',10000000)'
dfx canister --no-wallet call GoldenDIP20 approve  '(principal '\"$DEX_PRINCIPLE\"',10000000)'
# get ICP deposit address (removes unnesessary comma at the end)
ICP_DEPOSIT_ADDR=$(dfx canister call defi-dapp deposit_address |sed 's/\(.*\),/\1 /' | tr -d '\n')
# deposit some ICP in DEX
dfx canister call ledger transfer "(record { amount = record { e8s = 1000000 }; to = $ICP_DEPOSIT_ADDR; fee = record { e8s = 10000}; memo = 1;})"
# get token canister IDs
AKITA_ID=$(dfx canister --no-wallet id AkitaDIP20)
GOLDEN_ID=$(dfx canister --no-wallet id GoldenDIP20)
# deposit DIP. The amount that was approved
dfx canister call defi-dapp deposit_dip "(\"$AKITA_ID\")"
dfx canister call defi-dapp deposit_dip "(\"$GOLDEN_ID\")"
# transfer ICP to DEX
dfx canister call defi-dapp deposit_icp
```

### Deploy token

```bash

cd src/DIP20/
#remove old content
dfx stop
rm -rf .dfx
#create canisters
dfx canister --no-wallet create --all
# create principal idea that is inital owner of tokens
ROOT_HOME=$(mktemp -d)  
ROOT_PUBLIC_KEY="principal \"$(HOME=$ROOT_HOME dfx identity get-principal)\""
#build token canister
dfx build
# deploy token
dfx canister --no-wallet install DIP20 --argument="(\"https://dogbreedslist.com/wp-content/uploads/2019/08/Are-Golden-Retrievers-easy-to-train.png\", \"Golden Coin\", \"DOG\", 8, 10000000000000000, $ROOT_PUBLIC_KEY, 10000)"

# set fee structure. Need Home prefix since this is location of our identity
HOME=$ROOT_HOME  dfx canister  call DIP20 setFeeTo "($ROOT_PUBLIC_KEY)"
#deflationary
HOME=$ROOT_HOME dfx canister  call DIP20 setFee "(420)" 
# get balance. Congrats you are rich
HOME=$ROOT_HOME dfx canister --no-wallet call DIP20 balanceOf "($ROOT_PUBLIC_KEY)"
``` 


## Set allowance for DEX

should still be in `src/DIP20/`

```bash
#get principle ID of DEX
DEX_PRINCIPLE=$(dfx canister --no-wallet id defi-dapp)
# sth like this "r7inp-6aaaa-aaaaa-aaabq-cai"
# approve dex to spend on users behalf
HOME=$ROOT_HOME dfx canister --no-wallet call DIP20 approve  '(principal '\"$DEX_PRINCIPLE\"',10000)'
dfx canister --no-wallet call GoldenDIP20 approve  '(principal '\"$DEX_PRINCIPLE\"',1000000)'
dfx canister --no-wallet call AkitaDIP20 approve  '(principal '\"$DEX_PRINCIPLE\"',1000000)'

``` 




# Issues

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
-> `brew install cmake``

### broken M1 instruction link

(If you're running this on an M1 Mac, make sure you follow these steps) [here](https://github.com/dfinity/examples/tree/master/svelte-motoko-starter)

Do following:

```
cargo install ic-cdk-optimizer --version 0.3.1    

```
Change the II build file

```
https://github.com/dfinity/internet-identity/pull/434/files
```


### Access to localhost was denied


Change `dev` in `package.json`

````
  "scripts": {
    "build": "cd src/frontend && npm run build",
    "prebuild": "npm run copy:types",
    "dev": "cd src/frontend && HOST=0.0.0.0 npm run dev",
  }
```


