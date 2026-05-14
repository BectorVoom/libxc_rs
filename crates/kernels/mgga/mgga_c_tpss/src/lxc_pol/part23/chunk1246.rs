//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1246/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1246<F: Float>(t339: F, t5719: F, t790: F, t3277: F, t18464: F, t3350: F, t2376: F, t5726: F, t1250: F, t3354: F, t18480: F, t5570: F, t31297: F, t522: F, t18472: F, t219: F) -> (F, F, F, F, F, F, F, F, F) {
    let t60738 = t339 * t5719 * t790;
    let t60739 = t60738 * t3277;
    let t60744 = t18464 * t3350;
    let t60749 = t339 * t5726 * t2376;
    let t60750 = t60749 * t1250;
    let t60752 = t18464 * t3354;
    let t60778 = t18480 * t5570;
    let t60811 = t31297 * t522;
    let t60847 = t18472 * t219;
    (t60738, t60739, t60744, t60749, t60750, t60752, t60778, t60811, t60847)
}
