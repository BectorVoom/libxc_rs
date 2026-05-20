//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2640/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2640<F: Float>(t113: F, t12504: F, t12507: F, t12545: F, t12557: F, t1271: F, t12841: F, t16503: F, t2314: F, t2320: F, t2363: F, t4028: F, t4034: F, t4073: F, t4077: F, t45782: F, t46118: F, t50803: F, t510: F, t5107: F, t53757: F, t574: F, t652: F, t9348: F) -> F {
    let t53774 = -F::new(2.0) * t652 * t510 * t45782 - F::new(6.0) * t4034 * t12557 - F::new(6.0) * t652 * t5107 * t2363 - F::new(6.0) * t4028 * t12504 + t46118 * t574 - t113 * (t50803 + t53757) - F::new(6.0) * t4028 * t12507 - F::new(6.0) * t2314 * t12841 - F::new(12.0) * t2314 * t12545 - F::new(6.0) * t9348 * t4077 - F::new(6.0) * t9348 * t4073 - F::new(6.0) * t2320 * t5107 + F::new(3.0) * t1271 * t16503;
    t53774
}
