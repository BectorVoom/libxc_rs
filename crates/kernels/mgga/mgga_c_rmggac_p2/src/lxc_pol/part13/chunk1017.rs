//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1017/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1017<F: Float>(t2185: F, t678: F, t9086: F, t16043: F, t9051: F, t9055: F, t34847: F, t9123: F, t9213: F, t9218: F, t9106: F, t10792: F, t2301: F) -> (F, F, F, F, F, F, F, F) {
    let t42258 = t9086 * t2185 * t678;
    let t42260 = t16043 * t9051;
    let t42262 = t16043 * t9055;
    let t42264 = t34847 * t9123;
    let t42266 = t16043 * t9213;
    let t42268 = t16043 * t9218;
    let t42270 = t16043 * t9106;
    let t42272 = t10792 * t2301;
    (t42258, t42260, t42262, t42264, t42266, t42268, t42270, t42272)
}
