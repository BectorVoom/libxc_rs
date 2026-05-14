//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 859/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk859<F: Float>(t10292: F, t281: F, t283: F, t2403: F, t909: F, t241: F, t2978: F, t2967: F, t964: F, t340: F, t63: F, t344: F, t221: F, t339: F, t1032: F, t3082: F) -> (F, F, F, F, F, F, F, F) {
    let t10294 = t281 * t10292 * t283;
    let t10295 = 20.0 / 27.0 * t10294;
    let t10296 = t2403 * t909;
    let t10304 = t241 * t2978;
    let t10333 = t964 * t2967;
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = 0.3086419753086419753e-3 * t339 * t10337;
    let t10372 = t1032 * t3082;
    (t10294, t10295, t10296, t10304, t10333, t10335, t10339, t10372)
}
