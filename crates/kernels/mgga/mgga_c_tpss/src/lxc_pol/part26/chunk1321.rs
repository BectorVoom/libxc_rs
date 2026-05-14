//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1321/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1321<F: Float>(t69834: F, t69885: F, t70251: F, t70296: F, t1398: F, t14029: F, t14426: F, t1692: F, t1713: F, t18047: F, t18052: F, t19802: F, t21262: F, t21453: F, t2439: F, t3610: F, t36547: F, t3724: F, t4701: F, t4802: F, t4806: F, t52639: F, t5586: F, t5590: F, t60951: F, t60996: F, t6149: F, t64277: F, t69847: F, t69881: F, t70240: F, t70243: F, t70247: F, t821: F) -> (F, F) {
    let t70298 = t69834 + t69885 + t70251 + t70296;
    let t70733 = -2.0 * t1398 * t1692 * t64277 + 3.0 * t14029 * t1713 * t2439 - t14426 * t1692 * t5590 - t1692 * t18047 * t4802 + 4.0 * t1692 * t18052 * t69881 + 2.0 * t1692 * t18052 * t70240 - 2.0 * t1692 * t19802 * t3724 + 2.0 * t1692 * t4806 * t60996 - 6.0 * t1692 * t60951 * t70243 - t1692 * t70247 * t821 - 6.0 * t18047 * t21262 * t2439 + 6.0 * t18052 * t2439 * t69847 + 6.0 * t2439 * t3610 * t6149 + 3.0 * t2439 * t4701 * t5586 - 6.0 * t2439 * t52639 * t5590 + 6.0 * t21453 * t36547;
    (t70298, t70733)
}
