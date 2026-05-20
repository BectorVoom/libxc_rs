//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2149/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2149<F: Float>(t1992: F, t20018: F, t6976: F, t550: F, t57499: F, t22704: F, t22705: F, t28163: F, t57618: F, t1332: F, t19805: F, t2013: F, t28156: F, t81061: F, t81066: F, t81073: F, t81075: F, t81076: F, t90899: F, t90913: F, t93563: F, t97002: F, t97007: F, t97014: F) -> F {
    let t97017 = t1992 * t6976 * t20018;
    let t97023 = t1992 * t6976 * t57499 * t550;
    let t97026 = t22704 * t22705 * t28163;
    let t97030 = t1992 * t6976 * t57618 * t550;
    let t97032 = -F::cast_from(0.49348022005446793095e-1_f64) * t97002 - F::cast_from(0.63969658155208805863e-1_f64) * t81061 - F::cast_from(0.3289868133696452873e-1_f64) * t97007 - t90899 + t1332 * t28156 + t93563 + F::cast_from(0.82246703342411321824e-2_f64) * t81066 - t90913 - F::cast_from(0.19739208802178717238e0_f64) * t97014 - F::cast_from(0.16449340668482264365e-1_f64) * t97017 - t81073 - t81075 + F::cast_from(0.26044789391763585244e-1_f64) * t81076 + t19805 * t2013 - F::cast_from(0.16449340668482264365e-1_f64) * t97023 + F::cast_from(0.82246703342411321825e-2_f64) * t97026 - F::cast_from(0.82246703342411321825e-2_f64) * t97030;
    t97032
}
