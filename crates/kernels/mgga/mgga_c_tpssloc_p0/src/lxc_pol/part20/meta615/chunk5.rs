//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2221/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2221<F: Float>(t5: F, t12568: F, t12585: F, t12588: F, t12719: F, t1437: F, t2235: F, t2240: F, t2307: F, t39046: F, t39063: F, t3958: F, t4021: F, t45844: F, t46114: F, t9228: F, t9231: F, t9239: F, t9240: F) -> F {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t46116 = piecewise3::<F>(t8, F::new(0.0), F::new(840.0) * t1437 * t39063 * t9240 + F::new(60.0) * t2240 * t2307 * t4021 - F::new(360.0) * t2307 * t3958 * t9239 - F::new(12.0) * t12568 * t2307 + F::new(120.0) * t12585 * t9231 + F::new(60.0) * t12588 * t9231 - F::new(12.0) * t12719 * t2235 - F::new(4.0) * t1437 * t39046 - F::new(12.0) * t4021 * t9228 - F::new(120.0) * t45844 * t9240 + t46114);
    t46116
}
