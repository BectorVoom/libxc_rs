//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1260/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1260<F: Float>(t18464: F, t3354: F, t18480: F, t5570: F, t31297: F, t522: F, t18472: F, t219: F, t18532: F, t508: F, t1712: F, t31814: F, t2436: F, t580: F, t5585: F, t8096: F) -> (F, F, F, F, F, F, F, F) {
    let t60752 = t18464 * t3354;
    let t60778 = t18480 * t5570;
    let t60811 = t31297 * t522;
    let t60847 = t18472 * t219;
    let t60916 = t508 * t18532;
    let t60951 = t1712 * t31814;
    let t60960 = t2436 * t580;
    let t60996 = t5585 * t8096;
    (t60752, t60778, t60811, t60847, t60916, t60951, t60960, t60996)
}
