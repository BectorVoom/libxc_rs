//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 780/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk780<F: Float>(t2123: F, t4616: F, t265: F, t874: F, t876: F, t305: F, t7894: F, t942: F, t674: F, t7546: F, t7715: F, t20: F, t2018: F, t2021: F, t4729: F) -> (F, F, F, F, F, F, F) {
    let t36288 = t4616 * t2123;
    let t36292 = t874 * t265;
    let t36293 = t36292 * t876;
    let t36294 = t305 * t36293;
    let t36305 = t942 * t7894;
    let t36315 = t7546 * t7715 * t674;
    let t36330 = t4729 * t20 * t2018 * t2021;
    (t36288, t36292, t36293, t36294, t36305, t36315, t36330)
}
