//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2338/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2338<F: Float>(t13278: F, t5619: F, t1512: F, t59281: F, t1484: F, t16662: F, t16872: F, t16951: F, t20800: F, t20904: F, t20949: F, t20953: F, t2618: F, t2623: F, t2701: F, t4119: F, t41344: F, t4172: F, t4236: F, t46650: F, t46878: F, t5527: F, t5544: F, t5587: F, t58576: F, t776: F, t820: F, t843: F, t9607: F) -> F {
    let t67852 = t13278 * t5619;
    let t67854 = t59281 * t1512;
    let t67865 = -F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t58576 + t46650 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t4172 * t16951 + t46878 * t5587 / F::cast_from(512.0_f64) - F::cast_from(15.0_f64) / F::cast_from(128.0_f64) * t843 * t9607 * t820 * t5527 * t4119 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2623 * t20949 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t843 * t2701 * t820 * t4119 * t5544 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t843 * t2701 * t820 * t1484 * t16662 - t2618 * t20953 / F::cast_from(3072.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t67852 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t67854 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t843 * t2701 * t820 * t20800 * t776 - t41344 * t20904 / F::cast_from(512.0_f64) - t16872 * t4236 / F::cast_from(1024.0_f64);
    t67865
}
