//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 447/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk447<F: Float>(t1569: F, t339: F, t454: F, t1128: F, t1501: F, t242: F, t1097: F, t1098: F, t1111: F, t1122: F, t1125: F, t1554: F, t1558: F, t1564: F, t444: F, t463: F) -> (F, F, F) {
    let t1571 = t339 * t454 * t1569;
    let t1574 = t1128 * t1501;
    let t1575 = t242 * t1574;
    let t1578 = -t1554 * t444 / 36.0 + t1097 - t1098 * t1558 / 288.0 + t1111 * t1564 / 3072.0 - t1571 * t463 / 576.0 + t1122 - t1125 * t1575 / 4608.0;
    (t1571, t1575, t1578)
}
