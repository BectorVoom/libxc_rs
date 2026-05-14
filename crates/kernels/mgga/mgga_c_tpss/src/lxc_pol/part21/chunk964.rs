//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 964/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk964<F: Float>(t3486: F, t619: F, t1317: F, t2049: F, t1306: F, t1985: F, t1993: F, t3462: F, t582: F, t1289: F, t7737: F, t2009: F, t3431: F, t581: F, t1992: F, t3446: F) -> (F, F, F, F, F, F, F, F) {
    let t10306 = t3486 * t619;
    let t10309 = t1317 * t2049;
    let t10314 = t1985 * t1306;
    let t10317 = t1993 * t1306;
    let t10320 = t582 * t3462;
    let t10340 = t7737 * t1289 * t1985;
    let t10343 = t2009 * t3431;
    let t10344 = t10343 * t581;
    let t10347 = t3446 * t1992;
    (t10306, t10309, t10314, t10317, t10320, t10340, t10344, t10347)
}
