//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 816/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk816<F: Float>(t10471: F, t10474: F, t10470: F, t10482: F, t6739: F, t3127: F, t3131: F, t3215: F, t390: F, t268: F, t405: F, t6546: F, t154: F, t3584: F, t3241: F, t636: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11058 = t10471 * t10474;
    let t11059 = t10470 * t11058;
    let t11060 = t6739 * t10482;
    let t11064 = t10471 * t3127;
    let t11065 = t10470 * t11064;
    let t11066 = t6739 * t3131;
    let t11094 = 1.0 / t3215 / t390;
    let t11135 = t268 * t6546 * t405;
    let t11136 = 0.28842592592592592592e-1 * t11135;
    let t11145 = t154 * t3584;
    let t11147 = 1.0 / t3241 / t636;
    (t11059, t11060, t11065, t11066, t11094, t11135, t11136, t11145, t11147)
}
