//! Simplified matrix representations and operations.
//!
//! For more details, see `The anatomy of efficient matrix multipliers` blog post:
//! https://tech-blog.sonos.com/posts/the-anatomy-of-efficient-matrix-multipliers/

////////////////////////////////////////////////////////////////////////
// Matrix representation.

/// Represents a packed matrix of size `K x v128` of type `T` in row-major order
/// (elements in each row are stored contiguously in memory).
#[derive(Debug)]
pub struct Packed<T, const K: usize, const V128: usize>(pub [[T; V128]; K]);

/// Represents a matrix of size `M x N` of type `T` in row-major order
/// (elements in each row are stored contiguously in memory).
#[derive(Debug)]
pub struct Matrix<T, const M: usize, const N: usize>(pub [[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: From<u16> + Clone + Copy + Default,
{
    /// Creates a new `Matrix` of size `M x N`, filling it with numbers from
    /// range `0..M * N` (modulo `u16::MAX`).
    #[allow(clippy::needless_range_loop)]
    pub fn new() -> Matrix<T, M, N> {
        let mut new = [[T::default(); N]; M];
        for row in 0..M {
            for col in 0..N {
                // Make sure the compiler won't be able to predict the matrix content,
                // as it won't be able to speculate about the returned values.
                let black_box_zero = ic_cdk::api::time() - ic_cdk::api::time();
                // The conversion into `f32` is defined for up to `u16` integers.
                new[row][col] = ((col + row * N + black_box_zero as usize) as u16).into();
            }
        }
        Matrix(new)
    }

    /// Packs the matrix `A` into a slice of size `K x M`, assuming that
    /// the original matrix height `M` is exactly 128 bits.
    #[allow(clippy::needless_range_loop)]
    pub fn a_packed(&self) -> Packed<T, N, M>
    where
        T: From<u16> + Clone + Copy + Default,
    {
        // For simplicity, the original matrix height `M` is exactly 128 bits.
        assert_eq!(M * std::mem::size_of::<T>() * 8, 128);

        let mut res = [[T::default(); M]; N];
        for row in 0..N {
            for col in 0..M {
                res[row][col] = self.0[col][row];
            }
        }
        Packed(res)
    }

    /// Packs the matrix `B` into a slice of size `K x N`, assuming that
    /// the original matrix width `N` is exactly 128 bits.
    pub fn b_packed(&self) -> Packed<T, M, N>
    where
        T: From<u16> + Clone + Copy + Default,
    {
        // For simplicity, the original matrix width `N` is exactly 128 bits.
        assert_eq!(N * std::mem::size_of::<T>() * 8, 128);

        // No packing is needed if the width `N` is exactly 128 bits.
        Packed(self.0)
    }
}

////////////////////////////////////////////////////////////////////////
// Matrix-matrix multiplication algorithms.

/// Performs naive element-wise multiplication of matrices `A` of size `M x K`
/// and `B` of size `K x N`.
///
/// Returns the resulting `M x N` matrix with the element-wise products.
#[allow(clippy::needless_range_loop)]
pub fn add_mat_mul_naive<T, const M: usize, const K: usize, const N: usize>(
    a: &Matrix<T, M, K>,
    b: &Matrix<T, K, N>,
) -> Matrix<T, M, N>
where
    T: Copy + Default + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    let a = a.0;
    let b = b.0;
    let mut c = [[T::default(); N]; M];
    for col in 0..N {
        for row in 0..M {
            let mut sum = T::default();
            for k in 0..K {
                sum += a[row][k] * b[k][col];
            }
            c[row][col] = sum;
        }
    }
    Matrix(c)
}

/// Performs optimized element-wise multiplication of `K x 4` packed slices
/// from matrices `A` and `B`.
///
/// Returns the resulting `4 x 4` tile with the element-wise products.
#[inline(always)]
pub fn add_mat_mul_4x4_optimized<T>(k: usize, pa: *const u8, pb: *const u8) -> [[T; 4]; 4]
where
    T: Copy + Default + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    // Assert the 4 elements of `T` exactly fit into the 128 bits.
    assert_eq!(std::mem::size_of::<T>() * 4 * 8, 128);

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

/// Performs SIMD element-wise multiplication of `K x 4` packed slices
/// from matrices `A` and `B`.
///
/// Returns the resulting `4 x 4` tile with the element-wise products.
#[inline(always)]
pub fn add_mat_mul_4x4_simd_f32(k: usize, pa: *const u8, pb: *const u8) -> [[f32; 4]; 4] {
    use core::arch::wasm32::*;

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
            let b = v128_load(b.add(i));
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
