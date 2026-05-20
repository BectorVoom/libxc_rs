//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1640/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1640<F: Float>(t3298: F, t699: F, t3301: F, t3304: F, t241: F, t3439: F) -> (F, F, F, F) {
    let t11213 = t699 * t3298;
    let t11215 = t699 * t3301;
    let t11217 = t699 * t3304;
    let t11219 = t241 * t3439;
    (t11213, t11215, t11217, t11219)
}
