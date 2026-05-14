//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1210/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1210<F: Float>(t21052: F, t1705: F, t5427: F, t935: F, t1768: F, t5432: F, t18490: F, t1639: F, t1656: F, t520: F, t18497: F, t6255: F, t5740: F, t5448: F, t5380: F, t3260: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21053 = param_beta * t21052;
    let t21060 = t1705 * t5427;
    let t21061 = t21060 * t935;
    let t21069 = t1768 * t5432;
    let t21070 = t18490 * t21069;
    let t21074 = t1656 * t1639 * t520;
    let t21075 = t18497 * t21074;
    let t21078 = t6255 * t1656;
    let t21079 = t5740 * t21078;
    let t21082 = t1768 * t5448;
    let t21083 = t5740 * t21082;
    let t21086 = t1768 * t5380;
    let t21087 = t21086 * t3260;
    (t21053, t21060, t21061, t21069, t21070, t21074, t21075, t21078, t21079, t21082, t21083, t21086, t21087)
}
