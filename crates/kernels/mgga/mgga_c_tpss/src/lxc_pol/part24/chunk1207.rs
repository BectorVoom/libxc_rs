//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1207/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1207<F: Float>(t19466: F, t19479: F, t19491: F, t19588: F, t19693: F, t19706: F, t19718: F, t1625: F, t1659: F, t7029: F, t18547: F, t6243: F, t6277: F, t1270: F, t5371: F, t18538: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20142 = 7.0 / 72.0 * t19466;
    let t20146 = 7.0 / 1152.0 * t19479;
    let t20151 = 7.0 / 288.0 * t19491;
    let t20315 = 2.0 / 3.0 * t19588;
    let t20434 = 7.0 / 72.0 * t19693;
    let t20438 = 7.0 / 1152.0 * t19706;
    let t20443 = 7.0 / 288.0 * t19718;
    let t21011 = t1625 * t1659;
    let t21012 = t7029 * t21011;
    let t21014 = 6.0 * t18547 * t21012;
    let t21016 = 2.0 * t6243 * t6277;
    let t21017 = t1270 * t5371;
    let t21018 = t18538 * t21017;
    (t20142, t20146, t20151, t20315, t20434, t20438, t20443, t21011, t21012, t21014, t21016, t21017, t21018)
}
