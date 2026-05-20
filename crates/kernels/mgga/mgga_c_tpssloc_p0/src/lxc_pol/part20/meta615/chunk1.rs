//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2217/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2217<F: Float>(t12606: F, t607: F, t12648: F, t12649: F, t12652: F, t12653: F, t12661: F, t12709: F, t1434: F, t2252: F, t31: F, t4018: F, t45872: F, t45993: F, t45997: F, t628: F, t642: F, t65: F, t80: F, t9263: F) -> (F, F) {
    let t46006 = t607 * t12606;
    let t46022 = t12709 * t642 / F::new(8.0) - t9263 * t1434 / F::new(4.0) - t2252 * t4018 / F::new(4.0) - t45993 * t65 * t80 / F::new(12.0) - t45997 * t65 * t80 / F::new(4.0) - t12648 * t628 * t80 / F::new(4.0) - t12649 * t642 / F::new(4.0) - t46006 * t65 * t80 / F::new(4.0) - t12652 * t628 * t80 / F::new(2.0) - t12653 * t642 / F::new(2.0) - t31 * t45872 * t65 * t80 / F::new(12.0) - t12661 * t628 * t80 / F::new(4.0);
    (t46006, t46022)
}
