//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2765/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2765<F: Float>(t16752: F, t252: F, t4233: F, t232: F, t13170: F, t2632: F, t829: F, t13397: F, t13453: F, t16758: F, t16805: F, t16815: F, t16816: F, t17030: F, t17037: F, t2684: F, t40951: F, t4162: F, t4182: F, t4280: F, t4281: F, t4282: F, t4283: F, t4291: F, t58166: F, t812: F, t860: F, t863: F, t9632: F) -> (F, F, F, F, F) {
    let t58262 = t252 * t16752;
    let t58280 = t4233 * t4233;
    let t58281 = t58280 * t232;
    let t58289 = t2632 * t13170;
    let t58300 = t829 * t4233;
    let t58304 = -F::cast_from(24.0_f64) * t13397 * t16816 * t4233 * t4282 - F::cast_from(6.0_f64) * t13397 * t16815 * t40951 + F::cast_from(4.0_f64) * t16758 * t4281 * t9632 - t17030 * t2684 * t4291 + F::cast_from(2.0_f64) * t17030 * t4281 * t9632 + F::cast_from(8.0_f64) * t4162 * t4280 * t4283 + F::cast_from(12.0_f64) * t4182 * t4281 * t58166 + F::cast_from(4.0_f64) * t4182 * t4281 * t58262 + F::cast_from(4.0_f64) * t4281 * t4282 * t58289 - F::cast_from(4.0_f64) * t4282 * t4291 * t58300 - F::cast_from(2.0_f64) * t58281 * t812 * t860 + F::cast_from(8.0_f64) * t13453 * t17037 + F::cast_from(2.0_f64) * t16805 * t863;
    (t58262, t58280, t58281, t58289, t58304)
}
