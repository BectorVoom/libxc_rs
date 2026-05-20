//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1830/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1830<F: Float>(t25721: F, t3961: F, t25510: F, t23613: F, t7603: F, t1003: F, t1058: F, t23327: F, t23346: F, t23712: F, t25429: F, t25563: F, t25568: F, t25706: F, t25708: F, t25714: F, t25718: F, t3186: F, t353: F, t6680: F, t6687: F, t7604: F, t7615: F, t7622: F) -> (F, F, F, F) {
    let t25722 = t25721 * t3961;
    let t25723 = t25510 * t25722;
    let t25726 = t23613 * t7603;
    let t25729 = -F::cast_from(0.73108180748810063845e-2_f64) * t23346 * t7604 + F::cast_from(0.91385225936012579807e-3_f64) * t25563 - F::cast_from(0.21932454224643019153e-1_f64) * t6680 * t7615 + t1058 * t25568 + t1003 * t7622 + t353 * t25706 + F::new(2.0) * t3186 * t25708 + F::cast_from(0.91385225936012579807e-3_f64) * t23712 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25714 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t25718 + F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t25723 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t25726;
    (t25722, t25723, t25726, t25729)
}
