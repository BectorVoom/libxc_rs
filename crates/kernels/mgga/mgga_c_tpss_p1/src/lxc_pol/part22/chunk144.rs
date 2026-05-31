//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 144/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk144<F: Float>(t425: F, t434: F, t294: F, t410: F, t412: F, t421: F, t332: F, t56: F, t390: F) -> (F, F, F, F) {
    let t435 = t425 * t434;
    let t438 = t294 * (-F::cast_from(0.310907e-1_f64) * t412 * t421 + t410 - F::cast_from(0.19751673498613801407e-1_f64) * t435);
    let t440 = F::cast_from(0.19751673498613801407e-1_f64) * t294 * t435;
    let t441 = t56 * t332;
    let t442 = F::cast_from(1.0_f64) / t390;
    (t438, t440, t441, t442)
}
