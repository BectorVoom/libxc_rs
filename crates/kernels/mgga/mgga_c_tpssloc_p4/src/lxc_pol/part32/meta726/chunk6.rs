//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2347/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2347<F: Float>(t26012: F, t7974: F, t1860: F, t2109: F, t22549: F, t24514: F, t26009: F, t26024: F, t26028: F, t27303: F, t27308: F, t27365: F, t27956: F, t29481: F, t6486: F, t7255: F, t7428: F, t7975: F, t7978: F, t96045: F, t96379: F, t96458: F) -> F {
    let t104787 = t7974 * t26012;
    let t104813 = -F::new(10.0) * t96045 * t26009 - F::new(10.0) / F::new(3.0) * t22549 * t104787 - F::new(10.0) * t24514 * t96458 - t1860 * t7974 * t26024 / F::new(3.0) - t6486 * t29481 / F::new(6.0) - t1860 * t7255 * t27956 / F::new(6.0) - t1860 * t2109 * t96379 / F::new(6.0) - t26028 * t7975 / F::new(3.0) - t7428 * t27365 / F::new(3.0) - t7428 * t27303 / F::new(3.0) - t26028 * t7978 / F::new(3.0) - t7428 * t27308 / F::new(3.0);
    t104813
}
