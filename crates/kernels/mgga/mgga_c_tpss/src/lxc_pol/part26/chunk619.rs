//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 619/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk619<F: Float>(t1250: F, t3342: F, t508: F, t526: F, t235: F, t72: F, t1254: F, t219: F, t1257: F, t536: F, t73: F, t3255: F, t532: F, t1219: F, t1253: F, t507: F, t541: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3343 = t3342 * t1250;
    let t3346 = 1.0 / t526 / t508;
    let t3347 = t235 * t3346;
    let t3348 = t3347 * t72;
    let t3360 = t1254 * t219;
    let t3364 = 1.0 / t1257 / t536;
    let t3365 = t73 * t3364;
    let t3370 = t3255 * t532;
    let t3374 = t1219 * t1253;
    let t3391 = t507 * t541;
    (t3343, t3346, t3348, t3360, t3364, t3365, t3370, t3374, t3391)
}
