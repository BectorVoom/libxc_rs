//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1180/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1180<F: Float>(t61868: F, t507: F, t5935: F, t2: F, t823: F, t1288: F, t2436: F, t3678: F, t61033: F, t3638: F, t17954: F, t339: F, t3632: F, t790: F, t236: F, t61038: F) -> (F, F, F, F, F, F, F, F) {
    let t63006 = 308.0 / 27.0 * t61868;
    let t63042 = t507 * t5935;
    let t63783 = t823 * t2;
    let t63840 = t2436 * t1288;
    let t63907 = t61033 * t3678;
    let t63913 = t61033 * t3638;
    let t63917 = t339 * t17954 * t790 * t3632;
    let t63920 = t339 * t61038 * t236;
    (t63006, t63042, t63783, t63840, t63907, t63913, t63917, t63920)
}
