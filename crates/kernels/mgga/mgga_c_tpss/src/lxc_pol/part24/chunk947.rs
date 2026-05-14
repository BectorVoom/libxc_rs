//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 947/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk947<F: Float>(t1535: F, t2998: F, t1523: F, t2929: F, t2973: F, t11844: F, t11873: F, t11910: F, t11942: F, t11875: F, t3001: F, t4180: F, t1505: F, t2861: F, t1053: F, t4117: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12075 = t1535 * t2998;
    let t12083 = t1523 * t2929;
    let t12086 = t1535 * t2973;
    let t12093 = 0.11038e0 * t11844;
    let t12104 = 0.13418888888888888889e0 * t11873;
    let t12115 = 0.22076e0 * t11910;
    let t12129 = 0.20128333333333333334e0 * t11942;
    let t12145 = 0.2283111111111111111e-1 * t11875;
    let t12146 = 0.11415555555555555555e-1 * t11942;
    let t12210 = t4180 * t3001;
    let t12231 = 0.23744444444444444444e-1 * t11875;
    let t12232 = 0.11872222222222222222e-1 * t11942;
    let t12244 = t1505 * t2861;
    let t12264 = t4117 * t1053;
    (t12075, t12083, t12086, t12093, t12104, t12115, t12129, t12145, t12146, t12210, t12231, t12232, t12244, t12264)
}
