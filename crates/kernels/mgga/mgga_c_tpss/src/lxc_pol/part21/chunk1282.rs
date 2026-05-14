//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1282/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1282<F: Float>(t10596: F, t17964: F, t10795: F, t10799: F, t3678: F, t61033: F, t10805: F, t10581: F, t3638: F, t17954: F, t339: F, t3632: F, t790: F, t236: F, t61038: F, t10782: F) -> (F, F, F, F, F, F, F, F, F) {
    let t63901 = t17964 * t10596;
    let t63903 = t17964 * t10795;
    let t63905 = t17964 * t10799;
    let t63907 = t61033 * t3678;
    let t63908 = 7.0 / 288.0 * t63907;
    let t63909 = t17964 * t10805;
    let t63911 = t17964 * t10581;
    let t63913 = t61033 * t3638;
    let t63914 = 7.0 / 288.0 * t63913;
    let t63917 = t339 * t17954 * t790 * t3632;
    let t63918 = 7.0 / 576.0 * t63917;
    let t63920 = t339 * t61038 * t236;
    let t63921 = t63920 * t10782;
    (t63901, t63903, t63905, t63908, t63909, t63911, t63914, t63918, t63921)
}
