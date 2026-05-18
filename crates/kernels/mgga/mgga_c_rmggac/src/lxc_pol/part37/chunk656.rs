//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 656/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk656<F: Float>(t302: F, t7350: F, t22: F, t4616: F, t2078: F, t26: F, t3814: F, t265: F, t874: F, t507: F, t7191: F, t3924: F, t504: F) -> (F, F, F, F, F, F, F) {
    let t35718 = t7350 * t302;
    let t35928 = t4616 * t22;
    let t35959 = t2078 * t26;
    let t35960 = t3814 * t35959;
    let t36292 = t874 * t265;
    let t36471 = t507 * t7191;
    let t36596 = t504 * t3924;
    (t35718, t35928, t35959, t35960, t36292, t36471, t36596)
}
