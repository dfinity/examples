# WebAssembly SIMD example

Unlike other blockchains, the Internet Computer supports WebAssembly
SIMD ([Single Instruction, Multiple Data](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data))
instructions. This, combined with state-of-the-art Rust compiler support,
opens new horizons for the Internet Computer.

This example showcases different approaches to utilizing the new SIMD instructions: Rust auto-vectorization and SIMD intrinsics for matrix multiplication, a core operation in Machine Learning and Artificial Intelligence applications. The example compares various SIMD optimization techniques and their potential speedups.

The example consists of a canister named `mat_mat_mul` (matrix-matrix multiplication).

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/simd)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Run `dfx start --background --clean && dfx deploy` to deploy the project to your local environment. 

Compare the amount of instructions used for different matrix multiplication implementations. Call a loop performing 1K element-wise multiplications of `K x 4` packed slices
from matrices `A` and `B` using optimized algorithm, the same algorithm with Rust auto-vectorization enabled, and WebAssembly SIMD instructions:

```sh
dfx canister call mat_mat_mul optimized_f32
dfx canister call mat_mat_mul auto_vectorized_f32
dfx canister call mat_mat_mul simd_f32
```

In this example, Rust's auto-vectorization shines in optimizing matrix multiplication.
The auto-vectorized code achieves over 10x speedup compared to the optimized version!
Also, it's on par with the hand-crafted WebAssembly SIMD multiplication.

Compare the amount of instructions used for different matrix multiplication implementations. Call a loop performing 1K element-wise multiplications of `K x 4` packed slices
from matrices `A` and `B` using optimized algorithm and the same algorithm with Rust auto-vectorization enabled:

```sh
dfx canister call mat_mat_mul optimized_u32
dfx canister call mat_mat_mul auto_vectorized_u32
```

Rust auto-vectorization again demonstrates its power in this example. The auto-vectorized version of the integer matrix multiplication achieves more than a 2x speedup compared to the original code.

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
