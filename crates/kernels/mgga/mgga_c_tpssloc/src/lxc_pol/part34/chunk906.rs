//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 906/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk906<F: Float>(t243: F, t598: F, t213: F, t6584: F, t6604: F, t1891: F, t22822: F, t133: F, t6601: F, t6590: F) -> (F, F, F, F, F, F, F, F) {
    let t23075 = t243 * t243;
    let t23076 = 1.0 / t23075;
    let t23077 = t598 * t23076;
    let t23078 = t23077 * t213;
    let t23083 = t6584 * t6604;
    let t23093 = t22822 * t1891;
    let t23094 = t23093 * t133;
    let t23095 = t23094 * t6601;
    let t23097 = t6590 * t6604;
    (t23075, t23076, t23077, t23078, t23083, t23093, t23095, t23097)
}
