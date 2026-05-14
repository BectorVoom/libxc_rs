//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 490/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk490<F: Float>(t14286: F, t2079: F, t262: F, t3065: F, t3851: F, t328: F, t3814: F, t2566: F, t14173: F, t797: F, t27: F, t29: F, t352: F, t128: F, t1322: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14288 = t2079 * t262 * t14286;
    let t14290 = t3851 * t3065;
    let t14291 = t14290 * t328;
    let t14293 = t3814 * t3065;
    let t14294 = t14293 * t2566;
    let t14296 = t797 * t14173;
    let t14298 = t27 * t29 * t352;
    let t14299 = t14296 * t14298;
    let t14301 = t128 * t1322;
    (t14288, t14290, t14291, t14293, t14294, t14296, t14298, t14299, t14301)
}
