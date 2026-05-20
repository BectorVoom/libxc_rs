//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2141/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2141<F: Float>(t10236: F, t9288: F, t10427: F, t13969: F, t3130: F, t10432: F, t3039: F, t10943: F, t135: F, t973: F, t3152: F, t698: F) -> (F, F, F, F, F) {
    let t43075 = t10236 * t9288;
    let t43094 = t3130 * t13969 * t10427;
    let t43097 = t3039 * t13969 * t10432;
    let t43103 = t973 * t135 * t10943;
    let t43110 = t973 * t698 * t3152;
    (t43075, t43094, t43097, t43103, t43110)
}
