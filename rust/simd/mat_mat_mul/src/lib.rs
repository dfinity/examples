//! The example compares matrix multiplication using a naive algorithm,
//! optimized implementation, Rust loop auto-vectorization, and WebAssembly
//! SIMD instructions.
//!
//! The code is based on the Sonos `tract`, a Neural Network inference toolkit:
//! https://github.com/sonos/tract
//!
//! The matrix multiplication is typically encoded as a sequence of low-level
//! `[Clear, AddMatMul, Store, Done]` operators. For simplicity,
//! this example is focused only on the `AddMatMul` operator.
//!
//! # Performance Optimizations
//!
//! The multiplication of matrices `M x K` and `K x N` can be represented
//! as a series of smaller tiles multiplication. For simplicity, the example
//! focuses on a single tile multiplication, i.e. the multiplication of matrices
//! `4 x K` and `K x 4`.
//!
//! Also, the example uses the packing technique, rearranging the matrices
//! into slices of size `K x 4` before the multiplication.
//!
//! For more details, see `The anatomy of efficient matrix multipliers` blog post:
//! https://tech-blog.sonos.com/posts/the-anatomy-of-efficient-matrix-multipliers/
//!
//! See also the `Matrix multiplication algorithm` WikiPedia page:
//! https://en.wikipedia.org/wiki/Matrix_multiplication_algorithm
//!

mod mats;

/// The common dimension for multiplying two matrices of size `4 x K` and `K x 4`.
pub const K: usize = 224;

/// The number of iterations for tile multiplication.
pub const ITERATIONS: usize = 1_000;

/// Formats thousands for the specified `u64` integer (helper function).
fn fmt(n: u64) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("_")
}

////////////////////////////////////////////////////////////////////////
// Floating-point `f32` multiplications.

/// Returns the number of instructions used for a loop performing
/// 1K element-wise multiplications of matrices `A` and `B`
/// using naive algorithm.
#[ic_cdk_macros::query]
fn naive_f32() -> u64 {
    let a = mats::Matrix::<f32, 4, K>::new();
    let b = mats::Matrix::<f32, K, 4>::new();

    let instructions_before = ic_cdk::api::instruction_counter();
    for _ in 0..ITERATIONS {
        let c = mats::add_mat_mul_naive(&a, &b);
        std::hint::black_box(c);
    }
    let instructions = ic_cdk::api::instruction_counter() - instructions_before;

    ic_cdk::println!(
        "Naive algorithm (f32):     {:>12} Wasm instructions",
        fmt(instructions)
    );
    instructions
}

/// Returns the number of instructions used for a loop performing
/// 1K element-wise multiplications of `K x 4` packed slices
/// from matrices `A` and `B` using optimized algorithm.
#[ic_cdk_macros::query]
fn optimized_f32() -> u64 {
    let a = mats::Matrix::<f32, 4, K>::new();
    let b = mats::Matrix::<f32, K, 4>::new();
    let a_packed = a.a_packed();
    let b_packed = b.b_packed();
    let pa = a_packed.0.as_ptr() as *const u8;
    let pb = b_packed.0.as_ptr() as *const u8;

    let instructions_before = ic_cdk::api::instruction_counter();
    for _ in 0..ITERATIONS {
        let c = mats::add_mat_mul_4x4_optimized::<f32>(K, pa, pb);
        std::hint::black_box(c);
    }
    let instructions = ic_cdk::api::instruction_counter() - instructions_before;

    // Assert the naive and optimized algorithms produce the same results.
    assert_eq!(
        mats::add_mat_mul_naive(&a, &b).0,
        mats::add_mat_mul_4x4_optimized::<f32>(K, pa, pb)
    );

    ic_cdk::println!(
        "Optimized algorithm (f32): {:>12} Wasm instructions",
        fmt(instructions)
    );
    instructions
}

/// Returns the number of instructions used for a loop performing
/// 1K element-wise multiplications of `K x 4` packed slices
/// from matrices `A` and `B` using Rust loop auto-vectorization.
///
/// The following line enables auto-vectorization using WebAssembly SIMD instructions.
#[target_feature(enable = "simd128")]
#[ic_cdk_macros::query]
fn auto_vectorized_f32() -> u64 {
    let a = mats::Matrix::<f32, 4, K>::new();
    let b = mats::Matrix::<f32, K, 4>::new();
    let a_packed = a.a_packed();
    let b_packed = b.b_packed();
    let pa = a_packed.0.as_ptr() as *const u8;
    let pb = b_packed.0.as_ptr() as *const u8;

    let instructions_before = ic_cdk::api::instruction_counter();
    for _ in 0..ITERATIONS {
        let c = mats::add_mat_mul_4x4_optimized::<f32>(K, pa, pb);
        std::hint::black_box(c);
    }
    let instructions = ic_cdk::api::instruction_counter() - instructions_before;

    // Assert the naive and optimized algorithms produce the same results.
    assert_eq!(
        mats::add_mat_mul_naive(&a, &b).0,
        mats::add_mat_mul_4x4_optimized::<f32>(K, pa, pb)
    );

    ic_cdk::println!(
        "Auto-vectorized (f32):     {:>12} Wasm instructions",
        fmt(instructions)
    );
    instructions
}

/// Returns the number of instructions used for a loop performing
/// 1K element-wise multiplications of `K x 4` packed slices
/// from matrices `A` and `B` using WebAssembly SIMD instructions.
///
/// The following line enables WebAssembly SIMD instructions.
#[target_feature(enable = "simd128")]
#[ic_cdk_macros::query]
fn simd_f32() -> u64 {
    let a = mats::Matrix::<f32, 4, K>::new();
    let b = mats::Matrix::<f32, K, 4>::new();
    let a_packed = a.a_packed();
    let b_packed = b.b_packed();
    let pa = a_packed.0.as_ptr() as *const u8;
    let pb = b_packed.0.as_ptr() as *const u8;

    let instructions_before = ic_cdk::api::instruction_counter();
    for _ in 0..ITERATIONS {
        let c = mats::add_mat_mul_4x4_simd_f32(K, pa, pb);
        std::hint::black_box(c);
    }
    let instructions = ic_cdk::api::instruction_counter() - instructions_before;

    // Assert the naive and SIMD algorithms produce the same results.
    assert_eq!(
        mats::add_mat_mul_naive(&a, &b).0,
        mats::add_mat_mul_4x4_simd_f32(K, pa, pb)
    );

    ic_cdk::println!(
        "WebAssembly SIMD (f32):    {:>12} Wasm instructions",
        fmt(instructions)
    );
    instructions
}

////////////////////////////////////////////////////////////////////////
// Integer `u32` multiplications.

/// Returns the number of instructions used for a loop performing
/// 1K element-wise multiplications of matrices `A` and `B`
/// using naive algorithm.
#[ic_cdk_macros::query]
fn naive_u32() -> u64 {
    let a = mats::Matrix::<u32, 4, K>::new();
    let b = mats::Matrix::<u32, K, 4>::new();

    let instructions_before = ic_cdk::api::instruction_counter();
    for _ in 0..ITERATIONS {
        let c = mats::add_mat_mul_naive(&a, &b);
        std::hint::black_box(c);
    }
    let instructions = ic_cdk::api::instruction_counter() - instructions_before;

    ic_cdk::println!(
        "Naive algorithm (u32):     {:>12} Wasm instructions",
        fmt(instructions)
    );
    instructions
}

/// Returns the number of instructions used for a loop performing
/// 1K element-wise multiplications of `K x 4` packed slices
/// from matrices `A` and `B` using optimized algorithm.
#[ic_cdk_macros::query]
fn optimized_u32() -> u64 {
    let a = mats::Matrix::<u32, 4, K>::new();
    let b = mats::Matrix::<u32, K, 4>::new();
    let a_packed = a.a_packed();
    let b_packed = b.b_packed();
    let pa = a_packed.0.as_ptr() as *const u8;
    let pb = b_packed.0.as_ptr() as *const u8;

    let instructions_before = ic_cdk::api::instruction_counter();
    for _ in 0..ITERATIONS {
        let c = mats::add_mat_mul_4x4_optimized::<u32>(K, pa, pb);
        std::hint::black_box(c);
    }
    let instructions = ic_cdk::api::instruction_counter() - instructions_before;

    // Assert the naive and optimized algorithms produce the same results.
    assert_eq!(
        mats::add_mat_mul_naive(&a, &b).0,
        mats::add_mat_mul_4x4_optimized::<u32>(K, pa, pb)
    );

    ic_cdk::println!(
        "Optimized algorithm (u32): {:>12} Wasm instructions",
        fmt(instructions)
    );
    instructions
}

/// Returns the number of instructions used for a loop performing
/// 1K element-wise multiplications of `K x 4` packed slices
/// from matrices `A` and `B` using Rust loop auto-vectorization.
///
/// The following line enables auto-vectorization using WebAssembly SIMD instructions.
#[target_feature(enable = "simd128")]
#[ic_cdk_macros::query]
fn auto_vectorized_u32() -> u64 {
    let a = mats::Matrix::<u32, 4, K>::new();
    let b = mats::Matrix::<u32, K, 4>::new();
    let a_packed = a.a_packed();
    let b_packed = b.b_packed();
    let pa = a_packed.0.as_ptr() as *const u8;
    let pb = b_packed.0.as_ptr() as *const u8;

    let instructions_before = ic_cdk::api::instruction_counter();
    for _ in 0..ITERATIONS {
        let c = mats::add_mat_mul_4x4_optimized::<u32>(K, pa, pb);
        std::hint::black_box(c);
    }
    let instructions = ic_cdk::api::instruction_counter() - instructions_before;

    // Assert the naive and optimized algorithms produce the same results.
    assert_eq!(
        mats::add_mat_mul_naive(&a, &b).0,
        mats::add_mat_mul_4x4_optimized::<u32>(K, pa, pb)
    );

    ic_cdk::println!(
        "Auto-vectorized (u32):     {:>12} Wasm instructions",
        fmt(instructions)
    );
    instructions
}
