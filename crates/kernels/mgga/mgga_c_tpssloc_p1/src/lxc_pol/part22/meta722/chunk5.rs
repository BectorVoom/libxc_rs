//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2361/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2361<F: Float>(t12899: F, t16662: F, t1877: F, t20753: F, t20769: F, t20778: F, t39658: F, t40772: F, t4314: F, t4315: F, t46341: F, t46438: F, t5544: F, t67495: F, t67496: F, t67497: F, t67498: F, t868: F) -> F {
    let t68407 = -F::new(6.0) * t1877 * t20778 * t40772 * t868 + F::new(18.0) * t12899 * t4314 * t5544 + F::new(18.0) * t16662 * t4314 * t4315 + F::new(18.0) * t20753 * t46341 + F::new(18.0) * t20769 * t46341 - t39658 + t46438 + t67495 + t67496 + t67497 + t67498;
    t68407
}
