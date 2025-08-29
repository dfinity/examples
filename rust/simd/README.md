# WebAssembly SIMD Example

Unlike other blockchains, the Internet Computer supports WebAssembly
SIMD ([Single Instruction, Multiple Data](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data))
instructions. This, combined with state-of-the-art Rust compiler support,
opens new horizons for the Internet Computer.

This example showcases different approaches to utilizing the new SIMD instructions: Rust auto-vectorization and SIMD intrinsics for matrix multiplication, a core operation in Machine Learning and Artificial Intelligence applications. The example compares various SIMD optimization techniques and their potential speedups.

The example consists of a canister named `mat_mat_mul` (matrix-matrix multiplication).

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Run" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/simd)

## Build and deploy from the command-line

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install). Note: the WebAssembly SIMD support requires `dfx` version `0.20.2-beta.0` or later.
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

### Example 1: Floating point matrices multiplications

- #### Step 1: Setup project environment

Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```sh
cd examples/rust/simd
dfx start --clean
```

```sh
dfx start --clean
Running dfx start for version 0.20.2-beta.0
[...]
Dashboard: http://localhost:63387/_/dashboard
```

- #### Step 2: Open another terminal window in the same directory

```sh
cd examples/rust/simd
```

- #### Step 3: Compile and deploy `mat_mat_mul` canister

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

- #### Step 4: Compare the amount of instructions used for different matrix multiplication implementations

Call a loop performing 1K element-wise multiplications of `K x 4` packed slices
from matrices `A` and `B` using optimized algorithm, the same algorithm with
Rust auto-vectorization enabled, and WebAssembly SIMD instructions:

```sh
dfx canister call mat_mat_mul optimized_f32
dfx canister call mat_mat_mul auto_vectorized_f32
dfx canister call mat_mat_mul simd_f32
```

Example output:

```sh
% dfx canister call mat_mat_mul optimized_f32
(168_542_255 : nat64)
% dfx canister call mat_mat_mul auto_vectorized_f32
(13_697_228 : nat64)
% dfx canister call mat_mat_mul simd_f32
(13_697_228 : nat64)
```

In this example, Rust's auto-vectorization shines in optimizing matrix multiplication.
The auto-vectorized code achieves over 10x speedup compared to the optimized version!
Also, it's on par with the hand-crafted WebAssembly SIMD multiplication.

### Example 2: Integer matrices multiplications

- #### Step 1: Setup project environment

Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```sh
cd examples/rust/simd
dfx start --clean
```

```sh
dfx start --clean
Running dfx start for version 0.20.2-beta.0
[...]
Dashboard: http://localhost:63387/_/dashboard
```

- #### Step 2: Open another terminal window in the same directory

```sh
cd examples/rust/simd
```

- #### Step 3: Compile and deploy `mat_mat_mul` canister

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

- #### Step 4: Compare the amount of instructions used for different matrix multiplication implementations

Call a loop performing 1K element-wise multiplications of `K x 4` packed slices
from matrices `A` and `B` using optimized algorithm and the same algorithm
with Rust auto-vectorization enabled:

```sh
dfx canister call mat_mat_mul optimized_u32
dfx canister call mat_mat_mul auto_vectorized_u32
```

Example output:

```sh
% dfx canister call mat_mat_mul optimized_u32
(32_342_253 : nat64)
% dfx canister call mat_mat_mul auto_vectorized_u32
(16_164_254 : nat64)
```

Rust auto-vectorization again demonstrates its power in this example.
The auto-vectorized version of the integer matrix multiplication achieves
more than a 2x speedup compared to the original code.

## Further learning

1. Have a look at the locally running dashboard. The URL is at the end of the `dfx start` command: `Dashboard: http://localhost/...`
2. Check out `mat_mat_mul` canister Candid user interface. The URLs are at the end of the `dfx deploy` command: `mat_mat_mul: http://127.0.0.1/?canisterId=...`

### Canister interface

The `mat_mat_mul` canister provide the following interface:

- `naive_f32`/`naive_u32` &mdash;
  returns the number of instructions used for a loop performing
  1K element-wise multiplications of matrices `A` and `B`
  using naive algorithm.
- `optimized_f32`/`optimized_u32` &mdash;
  returns the number of instructions used for a loop performing
  1K element-wise multiplications of `K x 4` packed slices
  from matrices `A` and `B` using optimized algorithm.
- `auto_vectorized_f32`/`auto_vectorized_u32` &mdash;
  returns the number of instructions used for a loop performing
  1K element-wise multiplications of `K x 4` packed slices
  from matrices `A` and `B` using Rust loop auto-vectorization.
- `simd_f32` &mdash;
  Returns the number of instructions used for a loop performing
  1K element-wise multiplications of `K x 4` packed slices
  from matrices `A` and `B` using WebAssembly SIMD instructions.

Example usage:

```sh
dfx canister call mat_mat_mul naive_f32
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
