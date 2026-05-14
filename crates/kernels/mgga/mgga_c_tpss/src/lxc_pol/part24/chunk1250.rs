//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1250/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1250<F: Float>(t3678: F, t61033: F, t3638: F, t17954: F, t339: F, t3632: F, t790: F, t236: F, t61038: F, t3671: F, t1381: F, t61050: F, t1369: F, t61062: F, t17974: F, t3689: F) -> (F, F, F, F, F, F, F, F) {
    let t63907 = t61033 * t3678;
    let t63908 = 7.0 / 288.0 * t63907;
    let t63913 = t61033 * t3638;
    let t63914 = 7.0 / 288.0 * t63913;
    let t63917 = t339 * t17954 * t790 * t3632;
    let t63918 = 7.0 / 576.0 * t63917;
    let t63920 = t339 * t61038 * t236;
    let t63928 = t61033 * t3671;
    let t63929 = 7.0 / 1152.0 * t63928;
    let t63945 = t61050 * t1381;
    let t63957 = t61062 * t1369;
    let t63960 = t17974 * t3689;
    (t63908, t63914, t63918, t63920, t63929, t63945, t63957, t63960)
}
