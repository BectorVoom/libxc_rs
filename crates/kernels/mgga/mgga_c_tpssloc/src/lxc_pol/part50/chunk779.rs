//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 779/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk779<F: Float>(t1873: F, t88: F, t2018: F, t3701: F, t192: F, t533: F, t1390: F, t2229: F, t3: F, t2239: F, t601: F) -> (F, F, F, F, F, F, F) {
    let t8601 = t88 * t1873;
    let t8643 = t3701 * t2018;
    let t8944 = t192 * t533;
    let t8945 = t2018 * t1390;
    let t9222 = t2229 * t3;
    let t9223 = 1.0 / t9222;
    let t9231 = t601 * t2239;
    (t8601, t8643, t8944, t8945, t9222, t9223, t9231)
}
