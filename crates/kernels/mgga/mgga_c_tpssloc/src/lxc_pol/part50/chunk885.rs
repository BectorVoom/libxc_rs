//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 885/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk885<F: Float>(t25510: F, t25722: F, t23613: F, t7603: F, t1003: F, t1058: F, t23327: F, t23346: F, t23712: F, t25429: F, t25563: F, t25568: F, t25706: F, t25708: F, t25714: F, t25718: F, t3186: F, t353: F, t6680: F, t6687: F, t7604: F, t7615: F, t7622: F) -> (F,) {
    let t25723 = t25510 * t25722;
    let t25726 = t23613 * t7603;
    let t25729 = -0.73108180748810063845e-2 * t23346 * t7604 + 0.91385225936012579807e-3 * t25563 - 0.21932454224643019153e-1 * t6680 * t7615 + t1058 * t25568 + t1003 * t7622 + t353 * t25706 + 2.0 * t3186 * t25708 + 0.91385225936012579807e-3 * t23712 - 0.82246703342411321825e-2 * t6687 * t25714 + 0.27415567780803773942e-2 * t6687 * t25718 + 0.36554090374405031923e-2 * t25429 * t25723 - 0.27415567780803773942e-2 * t23327 * t25726;
    (t25729,)
}
