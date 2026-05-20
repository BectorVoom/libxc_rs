//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2346/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2346<F: Float>(t12988: F, t13005: F, t16771: F, t20756: F, t20800: F, t213: F, t221: F, t4119: F, t41200: F, t4127: F, t46770: F, t46772: F, t46783: F, t46847: F, t5544: F, t59154: F, t59156: F, t59165: F, t59173: F, t776: F) -> F {
    let t68102 = F::cast_from(0.13999999999999999999e0_f64) * t59154 - F::cast_from(0.69999999999999999996e-1_f64) * t59156 + F::cast_from(0.29999999999999999999e-1_f64) * t59165 - F::cast_from(0.14999999999999999999e-1_f64) * t59173 - F::cast_from(0.38888888888888888888e-1_f64) * t46770 + F::cast_from(0.98611111111111111109e-1_f64) * t46772 - t46783 - t41200 + F::cast_from(0.49999999999999999998e-2_f64) * t4127 * t221 * t213 * t20800 * t776 + F::cast_from(0.99999999999999999995e-1_f64) * t46847 * t221 * t213 * t20756 * t776 - F::cast_from(0.59999999999999999997e-1_f64) * t13005 * t221 * t16771 * t4119 + F::cast_from(0.14999999999999999999e-1_f64) * t4127 * t221 * t12988 * t5544;
    t68102
}
