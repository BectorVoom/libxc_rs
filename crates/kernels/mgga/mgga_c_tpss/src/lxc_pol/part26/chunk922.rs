//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 922/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk922<F: Float>(t9181: F, t235: F, t3032: F, t2839: F, t610: F, t1039: F, t2202: F, t57: F, t262: F, t390: F, t5543: F, t1016: F, t2193: F) -> (F, F, F, F, F, F, F, F) {
    let t9182 = 0.36793333333333333333e0 * t9181;
    let t9185 = t235 * t3032;
    let t9187 = 1.0 / t2839 / t610;
    let t9192 = t2202 * t1039;
    let t9198 = t2839 * t57;
    let t9199 = 1.0 / t9198;
    let t9213 = t262 * t5543 * t390;
    let t9214 = 0.93932222222222222223e0 * t9213;
    let t9221 = t2193 * t1016;
    (t9182, t9185, t9187, t9192, t9199, t9213, t9214, t9221)
}
