//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 939/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk939<F: Float>(t11453: F, t3955: F, t2731: F, t3978: F, t967: F, t3973: F, t2761: F, t8444: F, t3934: F, t2722: F, t140: F, t928: F, t3754: F, t925: F, t2697: F, t3749: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11454 = t11453 * t3955;
    let t11456 = t2731 * t11454 / 2304.0;
    let t11457 = t11453 * t3978;
    let t11459 = t967 * t11457 / 1728.0;
    let t11460 = t11453 * t3973;
    let t11462 = 5.0 / 10368.0 * t967 * t11460;
    let t11475 = t2761 * t8444;
    let t11506 = t11453 * t3934;
    let t11508 = t2722 * t11506 / 1152.0;
    let t11521 = t140 * t928;
    let t11522 = t11521 * t3754;
    let t11524 = t925 * t11522 / 216.0;
    let t11525 = t140 * t2697;
    let t11526 = t11525 * t3749;
    (t11454, t11456, t11457, t11459, t11460, t11462, t11475, t11506, t11508, t11524, t11526)
}
