# Counter canister in WebAssembly

This example demonstrates a counter application, written in WebAssembly directly. The compiled Wasm module is only 389 bytes.

It can be a good way to learn and experiment with [the IC system API](https://github.com/dfinity-lab/ic-ref/blob/0.17.0/spec/index.adoc#canister-interface-system-api).

## Prerequisites

Install the `wat2wasm` tool, which is part of [WABT](https://github.com/WebAssembly/wabt). 
For example, on Mac, you can run the following command:

```
$ brew install wabt
```

## Build

```
$ dfx start [--background]
$ dfx deploy [--no-wallet] counter

$ dfx canister call counter get
(0 : int64)
$ dfx canister call counter inc
()
$ dfx canister call counter get
(1 : int64)
$ dfx canister call counter set '(42)'
()
$ dfx canister call counter get
(42 : int64)
```
