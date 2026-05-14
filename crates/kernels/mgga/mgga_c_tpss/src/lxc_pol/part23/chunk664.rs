//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 664/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk664<F: Float>(t3073: F, t450: F, t1112: F, t242: F, t1108: F, t2713: F, t3050: F) -> (F, F, F) {
    let t3074 = t3073 * t450;
    let t3075 = t1112 * t3074;
    let t3076 = t242 * t3075;
    let t3080 = t2713 * t1108 * t3050;
    (t3074, t3076, t3080)
}
