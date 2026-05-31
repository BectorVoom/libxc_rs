//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 867/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk867<F: Float>(t2229: F, t3: F, t19: F, t2233: F, t604: F, t2239: F, t601: F, t83: F, t84: F, t85: F, t24: F, t41: F, t42: F) -> (F, F, F, F, F) {
    let t9222 = t2229 * t3;
    let t9223 = F::cast_from(1.0_f64) / t9222;
    let t9225 = F::cast_from(0.75936e3_f64) * t19 * t9223;
    let t9228 = t2233 * t604;
    let t9231 = t601 * t2239;
    let t9238 = F::cast_from(1.0_f64) / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    let t9287 = F::cast_from(1.0_f64) / t42 / t41;
    (t9225, t9228, t9231, t9239, t9287)
}
