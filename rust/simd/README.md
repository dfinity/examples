---
keywords: [advanced, rust, simd, ai]
---

# WebAssembly SIMD Example

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/simd)

## Overview

Unlike other blockchains, the Internet Computer supports WebAssembly
SIMD ([Single Instruction, Multiple Data](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data))
instructions. This, combined with state-of-the-art Rust compiler support,
opens new horizons for the Internet Computer.

This example showcases different approaches to utilizing the new SIMD instructions: Rust auto-vectorization and SIMD intrinsics for matrix multiplication, a core operation in Machine Learning and Artificial Intelligence applications. The example compares various SIMD optimization techniques and their potential speedups.

The example consists of a canister named `mat_mat_mul` (matrix-matrix multiplication).

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx). Note: the WebAssembly SIMD support requires `dfx` version `0.21` or later.
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

### Example 1: Floating point matrices multiplications

- #### Step 1.1: Begin by opening a terminal window and navigating into the project's directory

```sh
cd examples/rust/simd
```

- #### Step 1.2: Start a clean local Internet Computer replica and a web server

```sh
dfx stop
dfx start --clean
```

This terminal will stay blocked, printing log messages, until the `Ctrl+C` is pressed or `dfx stop` command is run.

Example output:

```sh
% dfx stop && dfx start --clean
Running dfx start for version 0.21.0
[...]
Dashboard: http://localhost:63387/_/dashboard
```

- #### Step 1.3: Open another terminal window in the same directory

```sh
cd examples/rust/simd
```

- #### Step 1.4: Compile and deploy `mat_mat_mul` canister

```sh
dfx deploy
```

Example output:

```sh
% dfx deploy
[...]
Deployed canisters.
URLs:
   Backend canister via Candid interface:
      mat_mat_mul: http://127.0.0.1/?canisterId=...
```

- #### Step 1.5: Compare the amount of instructions used for different matrix multiplication implementations

```sh
dfx canister call mat_mat_mul generic_f32
dfx canister call mat_mat_mul simd_f32
dfx canister call mat_mat_mul auto_vectorization_f32
```

Example output:

```sh
% dfx canister call mat_mat_mul generic_f32
(167_925_151 : nat64)
% dfx canister call mat_mat_mul simd_f32
(71_223_151 : nat64)
% dfx canister call mat_mat_mul auto_vectorization_f32
(12_925_207 : nat64)
```

In this example, Rust's auto-vectorization shines in optimizing matrix multiplication.
The auto-vectorized code achieves over 10x speedup compared to the original version!

It's important to note that the generic code's performance is currently limited
due to a known issue with NaN canonicalization in `wasmtime`.
This issue [has been fixed](https://github.com/bytecodealliance/wasmtime/commit/72a3b8b99d7c0343bacb7cd2cff3151b0144179d)
by DFINITY, but not yet released at the time of writing.

### Example 2: Integer matrices multiplications

- #### Step 2.1: Begin by opening a terminal window and navigating into the project's directory

```sh
cd examples/rust/simd
```

- #### Step 2.2: Start a clean local Internet Computer replica and a web server

```sh
dfx stop
dfx start --clean
```

This terminal will stay blocked, printing log messages, until the `Ctrl+C` is pressed or `dfx stop` command is run.

Example output:

```sh
% dfx stop && dfx start --clean
Running dfx start for version 0.21.0
[...]
Dashboard: http://localhost:63387/_/dashboard
```

- #### Step 2.3: Open another terminal window in the same directory

```sh
cd examples/rust/simd
```

- #### Step 2.4: Compile and deploy `mat_mat_mul` canister

```sh
dfx deploy
```

Example output:

```sh
% dfx deploy
[...]
Deployed canisters.
URLs:
   Backend canister via Candid interface:
      mat_mat_mul: http://127.0.0.1/?canisterId=...
```

- #### Step 2.5: Compare the amount of instructions used for different matrix multiplication implementations

```sh
dfx canister call mat_mat_mul generic_f32
dfx canister call mat_mat_mul auto_vectorization_f32
```

Example output:

```sh
% dfx canister call mat_mat_mul generic_f32
(31_484_435 : nat64)
% dfx canister call mat_mat_mul auto_vectorization_f32
(15_249_432 : nat64)
```

Rust auto-vectorization again demonstrates its power in this example!
The auto-vectorized version of the integer matrix multiplication achieves
more than a 2x speedup compared to the original code.

However, the slower performance of the generic floating-point
matrix multiplication is due to a known issue with NaN canonicalization in `wasmtime`.
This issue [has been fixed](https://github.com/bytecodealliance/wasmtime/commit/72a3b8b99d7c0343bacb7cd2cff3151b0144179d)
by DFINITY, but not yet released at the time of writing.

## Further learning

1. Have a look at the locally running dashboard. The URL is at the end of the `dfx start` command: `Dashboard: http://localhost/...`
2. Check out `mat_mat_mul` canister Candid user interface. The URLs are at the end of the `dfx deploy` command: `mat_mat_mul: http://127.0.0.1/?canisterId=...`

### Canister interface

The `mat_mat_mul` canister provide the following interface:

- `generic_f32` &mdash; returns the number of instructions used for a loop performing element-wise multiplication of `224x4` tiles of matrices `A` and `B` using generic Rust operators.
- `auto_vectorization_f32` &mdash; returns the number of instructions used for a loop performing element-wise multiplication of `224x4` tiles of matrices `A` and `B` using loop auto-vectorization for generic Rust operators.
- `simd_f32` &mdash; returns the number of instructions used for a loop performing element-wise multiplication of `224x4` tiles of matrices `A` and `B` using WebAssembly SIMD instructions.
- `generic_u32` &mdash; returns the number of instructions used for a loop performing element-wise multiplication of `224x4` tiles of integer matrices `A` and `B` using generic Rust operators.
- `auto_vectorization_u32` &mdash; returns the number of instructions used for a loop performing element-wise multiplication of `224x4` tiles of integer matrices `A` and `B` using loop auto-vectorization for generic Rust operators.

Example usage:

```sh
dfx canister call mat_mat_mul generic_f32
```

## Conclusion

WebAssembly SIMD instructions unlock new possibilities for the Internet Computer,
particularly in Machine Learning and Artificial Intelligence dApps. This example
demonstrates potential 10x speedups for matrix multiplication with minimal effort
using just Rust's loop auto-vectorization.

As shown in Example 2, integer operations also benefit, although with a more modest
"2x" speedup.

The actual speedups will vary depending on the specific application and the type
of operations involved.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize
yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/)
for developing on the Internet Computer. This example may not implement all the best practices.
