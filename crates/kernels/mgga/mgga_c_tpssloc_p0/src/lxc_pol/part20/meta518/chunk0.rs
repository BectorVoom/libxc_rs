//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2044/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2044<F: Float>(t39378: F, t746: F, t9720: F, t1294: F, t1285: F, t9214: F, t12451: F, t1390: F, t12132: F, t588: F, t39253: F, t702: F, t9453: F) -> (F, F, F, F, F, F) {
    let t39568 = t9720 * t39378 * t746;
    let t39570 = F::cast_from(0.14035736694323150897e2_f64) * t1294 * t39568;
    let t39571 = t9214 * t1285;
    let t39577 = t12451 * t1390;
    let t39581 = t588 * t12132;
    let t39585 = F::cast_from(24.0_f64) * t9453 * t39253 * t702;
    (t39568, t39570, t39571, t39577, t39581, t39585)
}
