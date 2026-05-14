//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1141/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1141<F: Float>(t226: F, t782: F, t818: F, t18007: F, t5562: F, t5572: F, t1702: F, t2425: F, t2157: F, t811: F, t2161: F, t2162: F, t5577: F, t2364: F, t1708: F, t17981: F, t228: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18009 = t818 * t782 * t226;
    let t18010 = t18007 * t18009;
    let t18014 = t5572 * t5562 * t818;
    let t18018 = t5572 * t1702 * t2425;
    let t18021 = t811 * t2157;
    let t18022 = t1702 * t2161;
    let t18023 = t18022 * t2162;
    let t18024 = t18021 * t18023;
    let t18028 = t5562 * t782 * t226;
    let t18029 = t5577 * t18028;
    let t18033 = t1702 * t2364 * t226;
    let t18034 = t5577 * t18033;
    let t18036 = t18022 * t226;
    let t18037 = t5577 * t18036;
    let t18040 = t1708 * t228 * t17981;
    (t18009, t18010, t18014, t18018, t18021, t18024, t18029, t18034, t18037, t18040)
}
