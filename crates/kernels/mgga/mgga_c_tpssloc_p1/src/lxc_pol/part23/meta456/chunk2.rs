//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1321/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1321<F: Float>(t232: F, t76085: F, t119: F, t1484: F, t16872: F, t20800: F, t20904: F, t20949: F, t210: F, t2630: F, t2701: F, t41139: F, t41349: F, t4172: F, t46957: F, t47047: F, t5614: F, t5619: F, t68021: F, t75978: F, t76002: F, t76074: F, t76086: F, t787: F, t817: F, t819: F, t820: F, t843: F) -> (F, F) {
    let t76327 = t76085 * t232;
    let t76333 = -t46957 * t20904 / F::cast_from(128.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t4172 * t20949 - t16872 * t5614 / F::cast_from(512.0_f64) - t787 * t210 * t119 * t75978 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t68021 + t2630 * t819 * t820 * t76002 / F::cast_from(512.0_f64) + t41349 * t819 * t820 * t76086 / F::cast_from(128.0_f64) - t16872 * t5619 / F::cast_from(512.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t843 * t2701 * t820 * t20800 * t1484 - t817 * t819 * t820 * t76074 / F::cast_from(3072.0_f64) - t817 * t819 * t820 * t76327 / F::cast_from(3072.0_f64) - F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t47047 + t41139;
    (t76327, t76333)
}
