//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2662/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2662<F: Float>(t2239: F, t5385: F, t12568: F, t12582: F, t12719: F, t1437: F, t16: F, t19313: F, t19445: F, t2240: F, t2241: F, t2307: F, t39033: F, t39037: F, t39043: F, t39049: F, t3953: F, t3958: F, t4021: F, t45844: F, t46099: F, t5389: F, t5445: F, t645: F, t86: F, t9231: F, t9239: F) -> F {
    let t55921 = t5385 * t2239;
    let t55924 = -F::cast_from(8.0_f64) * t46099 * t1437 - F::cast_from(16.0_f64) * t12568 * t4021 + F::cast_from(40.0_f64) * t2240 * t1437 * t12719 - F::cast_from(240.0_f64) * t45844 * t12582 - F::cast_from(120.0_f64) * t9239 * t5445 * t2241 - F::cast_from(8.0_f64) * t3953 * t12719 + F::cast_from(80.0_f64) * t9231 * t19313 + F::cast_from(40.0_f64) * t2240 * t19445 * t645 + F::cast_from(20.0_f64) * t2240 * t5445 * t2307 + F::cast_from(20.0_f64) * t39049 * t5389 - F::cast_from(480.0_f64) * t9239 * t3958 * t4021 + (-F::cast_from(0.888e1_f64) * t16 + F::cast_from(678.0_f64) * t39033 - F::cast_from(0.52752e4_f64) * t39037 + t39043) * t86 + F::cast_from(20.0_f64) * t55921 * t2241;
    t55924
}
