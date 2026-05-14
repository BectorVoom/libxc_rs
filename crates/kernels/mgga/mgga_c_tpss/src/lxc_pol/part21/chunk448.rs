//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 448/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk448<F: Float>(t1013: F, t1501: F, t128: F, t1012: F, t408: F, t1011: F) -> (F, F, F, F, F) {
    let t1502 = t1013 * t1501;
    let t1503 = t128 * t1502;
    let t1505 = -t1012 + 0.17808333333333333333e-1 * t1503;
    let t1507 = 0.621814e-1 * t1505 * t408;
    let t1509 = -t1011 / 3.0 + t1503 / 3.0;
    (t1502, t1503, t1505, t1507, t1509)
}
