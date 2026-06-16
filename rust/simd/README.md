# WebAssembly SIMD Example

Unlike other blockchains, the Internet Computer supports WebAssembly
SIMD ([Single Instruction, Multiple Data](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data))
instructions. This, combined with state-of-the-art Rust compiler support,
opens new horizons for the Internet Computer.

This example showcases different approaches to utilizing WebAssembly SIMD instructions: Rust auto-vectorization and SIMD intrinsics for matrix multiplication, a core operation in Machine Learning and Artificial Intelligence applications. The example compares various optimization techniques and their potential speedups.

The canister exposes seven query methods that each perform 1,000 iterations of a 4×K matrix multiplication tile and return the number of Wasm instructions consumed:

- `naive_f32` / `naive_u32` — straightforward element-wise loop
- `optimized_f32` / `optimized_u32` — uses packed slices to improve memory access patterns
- `auto_vectorized_f32` / `auto_vectorized_u32` — same optimized loop annotated with `#[target_feature(enable = "simd128")]`, letting Rust auto-vectorize with SIMD instructions
- `simd_f32` — hand-written SIMD intrinsics using `core::arch::wasm32`

In practice, Rust's auto-vectorization achieves over 10× speedup for f32 and more than 2× for u32 compared to the optimized baseline, and the hand-crafted SIMD version is on par with auto-vectorization.

The code is based on the Sonos `tract` Neural Network inference toolkit's `AddMatMul` operator. For more details on the packing technique, see [The anatomy of efficient matrix multipliers](https://tech-blog.sonos.com/posts/the-anatomy-of-efficient-matrix-multipliers/).

Note: SIMD instructions are selectively enabled per function using `#[target_feature(enable = "simd128")]` rather than globally. To enable SIMD for the whole workspace, uncomment the `rustflags` line in `.cargo/config.toml`.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/simd
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

### Comparing instruction counts

After deploying, call each method to see the instruction counts for different implementations:

```bash
# Floating-point matrix multiplication
icp canister call --query backend optimized_f32 '()'
icp canister call --query backend auto_vectorized_f32 '()'
icp canister call --query backend simd_f32 '()'

# Integer matrix multiplication
icp canister call --query backend optimized_u32 '()'
icp canister call --query backend auto_vectorized_u32 '()'
```

Example output:

```
# Floating-point results
(168_542_255 : nat64)   # optimized_f32
(13_697_228 : nat64)    # auto_vectorized_f32  — over 10x speedup!
(13_697_228 : nat64)    # simd_f32             — on par with auto-vectorization

# Integer results
(32_342_253 : nat64)    # optimized_u32
(16_164_254 : nat64)    # auto_vectorized_u32  — more than 2x speedup
```

Rust's auto-vectorization achieves over 10x speedup for float matrix multiplication compared to the optimized version, and it's on par with hand-crafted WebAssembly SIMD intrinsics. Integer operations also benefit with more than 2x speedup.

The actual speedups will vary depending on the specific application and the type of operations involved.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize
yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview)
for developing on the Internet Computer. This example may not implement all the best practices.
