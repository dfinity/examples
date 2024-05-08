//! Matrix multiplication using the generic Rust operators,
//! Rust loop auto-vectorization, and WebAssembly SIMD instructions.
//!
//! The `MatMatMul` is typically encoded as a sequence of low-level
//! `[Clear, AddMatMul, Store, Done]` operators. For simplicity,
//! we focus mainly on the `AddMatMul` operator, but elements of
//! `Clear` and `Store` operations are also involved.

mod mats;

/// Defines the common matrix dimension. We multiply two matrices of size
/// `M x K` and `K x N`. `K` must be a multiple of 4, as we perform
/// the multiplication on 4x4 tiles.
pub const K: usize = 224;

/// Defines the number of iterations for matrix multiplication.
pub const ITERATIONS: usize = 1_000;

/// Performs element-wise multiplication between `224x4` tiles of `A` and `B`
/// using generic Rust operators.
///
/// Returns the resulting `4x4` tile with the element-wise products.
fn add_mat_mul_4x4_generic<T>(k: usize, pa: *const u8, pb: *const u8) -> [[T; 4]; 4]
where
    T: Copy + Default + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    // `Clear` op.
    let mut ab = [[T::default(); 4]; 4];
    // `AddMatMul` + `Store` ops.
    unsafe {
        let a = pa as *const T;
        let b = pb as *const T;
        for i in 0..k {
            let a = std::slice::from_raw_parts(a.offset(4 * i as isize), 4);
            let b = std::slice::from_raw_parts(b.offset(4 * i as isize), 4);
            ab[0][0] += a[0] * b[0];
            ab[0][1] += a[0] * b[1];
            ab[0][2] += a[0] * b[2];
            ab[0][3] += a[0] * b[3];
            ab[1][0] += a[1] * b[0];
            ab[1][1] += a[1] * b[1];
            ab[1][2] += a[1] * b[2];
            ab[1][3] += a[1] * b[3];
            ab[2][0] += a[2] * b[0];
            ab[2][1] += a[2] * b[1];
            ab[2][2] += a[2] * b[2];
            ab[2][3] += a[2] * b[3];
            ab[3][0] += a[3] * b[0];
            ab[3][1] += a[3] * b[1];
            ab[3][2] += a[3] * b[2];
            ab[3][3] += a[3] * b[3];
        }
    }
    ab
}

/// Performs element-wise multiplication between `224x4` tiles of `A` and `B`
/// using WebAssembly SIMD instructions.
///
/// Returns the resulting `4x4` tile with the element-wise products.
pub fn add_mat_mul_4x4_simd_f32(k: usize, pa: *const u8, pb: *const u8) -> [[f32; 4]; 4] {
    use std::arch::wasm32::*;
    // `Clear` op.
    let mut ab0 = f32x4(0_f32, 0_f32, 0_f32, 0_f32);
    let mut ab1 = f32x4(0_f32, 0_f32, 0_f32, 0_f32);
    let mut ab2 = f32x4(0_f32, 0_f32, 0_f32, 0_f32);
    let mut ab3 = f32x4(0_f32, 0_f32, 0_f32, 0_f32);
    let mut ab = [[0.0; 4]; 4];

    // `AddMatMul` op.
    unsafe {
        let a = pa as *const f32;
        let b = pb as *const v128;
        for i in 0..k {
            let a = std::slice::from_raw_parts(a.offset(4 * i as isize), 4);
            let b = v128_load(b.offset(i as isize));
            ab0 = f32x4_add(ab0, f32x4_mul(f32x4_splat(a[0]), b));
            ab1 = f32x4_add(ab1, f32x4_mul(f32x4_splat(a[1]), b));
            ab2 = f32x4_add(ab2, f32x4_mul(f32x4_splat(a[2]), b));
            ab3 = f32x4_add(ab3, f32x4_mul(f32x4_splat(a[3]), b));
        }
    }

    // `Store` op.
    unsafe {
        v128_store(ab[0].as_mut_ptr() as *mut v128, ab0);
        v128_store(ab[1].as_mut_ptr() as *mut v128, ab1);
        v128_store(ab[2].as_mut_ptr() as *mut v128, ab2);
        v128_store(ab[3].as_mut_ptr() as *mut v128, ab3);
    }

    ab
}

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
/// element-wise multiplication of `224x4` tiles of matrices `A` and `B`
/// using generic Rust operators.
#[ic_cdk_macros::query]
fn generic_f32() -> u64 {
    let a = mats::M_4X4_F32.repeat(K / 4);
    let b = mats::M_4X4_F32.repeat(K / 4);
    let pa = a.as_ptr() as *const u8;
    let pb = b.as_ptr() as *const u8;

    for _ in 0..ITERATIONS {
        let res = add_mat_mul_4x4_generic::<f32>(K, pa, pb);
        std::hint::black_box(res);
    }
    // Assert the generic and SIMD matrix multiplication results are the same.
    assert_eq!(
        add_mat_mul_4x4_generic::<f32>(K, pa, pb),
        add_mat_mul_4x4_simd_f32(K, pa, pb)
    );

    let counter = ic_cdk::api::performance_counter(0);
    ic_cdk::println!(
        "Generic WebAssembly (f32): {:>12} Wasm instructions",
        fmt(counter)
    );
    counter
}

/// Returns the number of instructions used for a loop performing
/// element-wise multiplication of `224x4` tiles of matrices `A` and `B`
/// using loop auto-vectorization for generic Rust operators.
///
/// The following line enables auto-vectorization using WebAssembly SIMD instructions.
#[target_feature(enable = "simd128")]
#[ic_cdk_macros::query]
fn auto_vectorization_f32() -> u64 {
    let a = mats::M_4X4_F32.repeat(K / 4);
    let b = mats::M_4X4_F32.repeat(K / 4);
    let pa = a.as_ptr() as *const u8;
    let pb = b.as_ptr() as *const u8;

    for _ in 0..ITERATIONS {
        let res = add_mat_mul_4x4_generic::<f32>(K, pa, pb);
        std::hint::black_box(res);
    }
    // Assert the generic and SIMD matrix multiplication results are the same.
    assert_eq!(
        add_mat_mul_4x4_generic::<f32>(K, pa, pb),
        add_mat_mul_4x4_simd_f32(K, pa, pb)
    );

    let counter = ic_cdk::api::performance_counter(0);
    ic_cdk::println!(
        "Auto-vectorization (f32):  {:>12} Wasm instructions",
        fmt(counter)
    );
    counter
}

/// Returns the number of instructions used for a loop performing
/// element-wise multiplication of `224x4` tiles of matrices `A` and `B`
/// using WebAssembly SIMD instructions.
#[ic_cdk_macros::query]
fn simd_f32() -> u64 {
    let a = mats::M_4X4_F32.repeat(K / 4);
    let b = mats::M_4X4_F32.repeat(K / 4);
    let pa = a.as_ptr() as *const u8;
    let pb = b.as_ptr() as *const u8;

    for _ in 0..ITERATIONS {
        let res = add_mat_mul_4x4_simd_f32(K, pa, pb);
        std::hint::black_box(res);
    }
    // Assert the generic and SIMD matrix multiplication results are the same.
    assert_eq!(
        add_mat_mul_4x4_generic::<f32>(K, pa, pb),
        add_mat_mul_4x4_simd_f32(K, pa, pb)
    );

    let counter = ic_cdk::api::performance_counter(0);
    ic_cdk::println!(
        "WebAssembly SIMD (f32):    {:>12} Wasm instructions",
        fmt(counter)
    );
    counter
}

////////////////////////////////////////////////////////////////////////
// Integer `u32` multiplications.

/// Returns the number of instructions used for a loop performing
/// element-wise multiplication of `224x4` tiles of integer matrices `A` and `B`
/// using generic Rust operators.
#[ic_cdk_macros::query]
fn generic_u32() -> u64 {
    let a = mats::M_4X4_U32.repeat(K / 4);
    let b = mats::M_4X4_U32.repeat(K / 4);
    let pa = a.as_ptr() as *const u8;
    let pb = b.as_ptr() as *const u8;

    for _ in 0..ITERATIONS {
        let res = add_mat_mul_4x4_generic::<u32>(K, pa, pb);
        std::hint::black_box(res);
    }

    let counter = ic_cdk::api::performance_counter(0);
    ic_cdk::println!(
        "Generic WebAssembly (u32): {:>12} Wasm instructions",
        fmt(counter)
    );
    counter
}

/// Returns the number of instructions used for a loop performing
/// element-wise multiplication of `224x4` tiles of integer matrices `A` and `B`
/// using loop auto-vectorization for generic Rust operators.
///
/// The following line enables auto-vectorization using WebAssembly SIMD instructions.
#[target_feature(enable = "simd128")]
#[ic_cdk_macros::query]
fn auto_vectorization_u32() -> u64 {
    let a = mats::M_4X4_U32.repeat(K / 4);
    let b = mats::M_4X4_U32.repeat(K / 4);
    let pa = a.as_ptr() as *const u8;
    let pb = b.as_ptr() as *const u8;

    for _ in 0..ITERATIONS {
        let res = add_mat_mul_4x4_generic::<u32>(K, pa, pb);
        std::hint::black_box(res);
    }
    let counter = ic_cdk::api::performance_counter(0);
    ic_cdk::println!(
        "Auto-vectorization (u32):  {:>12} Wasm instructions",
        fmt(counter)
    );
    counter
}
