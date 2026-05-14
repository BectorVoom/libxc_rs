//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1177/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1177<F: Float>(t3247: F, t60724: F, t17942: F, t517: F, t1215: F, t18436: F, t3251: F, t339: F, t5719: F, t790: F, t3277: F, t18464: F, t3350: F, t2376: F, t5726: F, t1250: F) -> (F, F, F, F, F, F, F, F, F) {
    let t60725 = t60724 * t3247;
    let t60730 = t17942 * t517;
    let t60731 = t60730 * t1215;
    let t60733 = t18436 * t3251;
    let t60738 = t339 * t5719 * t790;
    let t60739 = t60738 * t3277;
    let t60744 = t18464 * t3350;
    let t60749 = t339 * t5726 * t2376;
    let t60750 = t60749 * t1250;
    (t60725, t60730, t60731, t60733, t60738, t60739, t60744, t60749, t60750)
}
