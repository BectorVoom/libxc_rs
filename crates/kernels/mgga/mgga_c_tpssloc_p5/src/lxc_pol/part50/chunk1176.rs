//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1176/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1176<F: Float>(t118582: F, t118615: F, t23270: F, t25038: F, t30622: F, t4255: F, t22986: F, t4119: F, t32814: F, t81651: F, t82074: F, t118510: F, t118518: F, t118523: F, t118526: F, t1527: F, t218: F, t25168: F, t25169: F, t25183: F, t25199: F, t25200: F, t259: F, t2718: F, t30651: F, t30728: F, t30729: F, t30741: F, t32852: F, t4147: F, t4273: F, t6627: F, t855: F, t865: F, t866: F) -> (F, F) {
    let t118616 = t118582 + t118615;
    let t118626 = F::cast_from(0.9869604401089358619e-1_f64) * t25038 * t23270 * t30622 * t4255;
    let t118630 = F::cast_from(0.3289868133696452873e-1_f64) * t22986 * t23270 * t30622 * t4119;
    let t118632 = t81651 * t82074 * t32814;
    let t118633 = F::cast_from(0.16449340668482264365e-1_f64) * t118632;
    let t118634 = F::cast_from(2.0_f64) * t1527 * t2718 * t30728 * t855 + F::cast_from(2.0_f64) * t2718 * t32852 * t855 * t865 + t118616 * t218 * t259 - F::cast_from(12.0_f64) * t25168 * t25169 * t25183 - F::cast_from(12.0_f64) * t25168 * t25169 * t25199 - t118510 * t866 + F::cast_from(4.0_f64) * t25200 * t6627 - F::cast_from(6.0_f64) * t30651 * t4147 - t30729 * t4147 + F::cast_from(2.0_f64) * t30741 * t4273 - t118518 - t118523 - t118526 - t118626 + t118630 - t118633;
    (t118616, t118634)
}
