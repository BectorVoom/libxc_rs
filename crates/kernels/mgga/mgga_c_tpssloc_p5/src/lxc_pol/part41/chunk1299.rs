//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1299/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1299<F: Float>(t1266: F, t1268: F, t12725: F, t1774: F, t19451: F, t19456: F, t20098: F, t2199: F, t2200: F, t2202: F, t2314: F, t28002: F, t30272: F, t30274: F, t30315: F, t30326: F, t30534: F, t30574: F, t4028: F, t4034: F, t55943: F, t6287: F, t652: F, t7458: F, t8176: F, t8189: F, t8194: F, t8260: F, t8274: F, t8278: F, t8280: F, t96356: F, t96683: F) -> F {
    let t111961 = -F::cast_from(2.0_f64) * t55943 * t2200 - F::cast_from(4.0_f64) * t96356 * t2200 - F::cast_from(4.0_f64) * t28002 * t8176 + F::cast_from(4.0_f64) * t12725 * t8278 + F::cast_from(4.0_f64) * t96356 * t2202 + F::cast_from(4.0_f64) * t28002 * t8194 + F::cast_from(4.0_f64) * t12725 * t8280 - F::cast_from(4.0_f64) * t19456 * t8274 - F::cast_from(4.0_f64) * t4028 * t30272 + F::cast_from(2.0_f64) * t19451 * t8194 + F::cast_from(2.0_f64) * t1268 * t2199 * t20098 - F::cast_from(4.0_f64) * t19456 * t8260 - F::cast_from(4.0_f64) * t4028 * t30326 - F::cast_from(4.0_f64) * t7458 * t30274 - F::cast_from(2.0_f64) * t2314 * t30574 - F::cast_from(2.0_f64) * t4034 * t30574 - F::cast_from(2.0_f64) * t652 * t1266 * t30534 - F::cast_from(2.0_f64) * t652 * t6287 * t8189 - F::cast_from(4.0_f64) * t96683 * t2200 - F::cast_from(4.0_f64) * t652 * t1774 * t30315;
    t111961
}
