//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2720/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2720<F: Float>(t1307: F, t1365: F, t16018: F, t16186: F, t16192: F, t16196: F, t16199: F, t19631: F, t19708: F, t19715: F, t19716: F, t19719: F, t19724: F, t225: F, t3719: F, t3734: F, t3844: F, t5272: F, t5278: F, t5279: F, t5280: F, t548: F, t57193: F, t57194: F, t57196: F, t57197: F, t57200: F, t57201: F, t57217: F, t57238: F, t6330: F, t6404: F, t68: F, t6924: F) -> F {
    let t57266 = -F::cast_from(48.0_f64) * t16186 * t19719 - F::cast_from(48.0_f64) * t5272 * t68 * t5280 - F::cast_from(48.0_f64) * t19708 * t16196 - F::cast_from(24.0_f64) * t5278 * t5279 * t16018 - (t57193 + t57194 + t57196 + t57197 + t57200 + t57201 + t57217 + t57238) * t225 * t548 - F::cast_from(360.0_f64) * t5278 * t6924 * t6330 * t3734 + F::cast_from(60.0_f64) * t5278 * t19715 * t3719 + F::cast_from(120.0_f64) * t16186 * t19716 - F::cast_from(24.0_f64) * t5278 * t1365 * t19631 * t1307 - F::cast_from(12.0_f64) * t5278 * t19724 * t3719 + F::cast_from(120.0_f64) * t19708 * t16192 - F::cast_from(24.0_f64) * t19708 * t16199 - F::cast_from(12.0_f64) * t6404 * t3844;
    t57266
}
