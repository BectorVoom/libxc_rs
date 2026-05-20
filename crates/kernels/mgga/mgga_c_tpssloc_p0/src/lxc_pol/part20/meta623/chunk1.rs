//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2244/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2244<F: Float>(t13115: F, t9932: F, t32: F, t4094: F, t2659: F, t1530: F, t193: F, t39658: F, t46426: F, t46432: F, t46434: F, t46436: F, t46438: F, t46439: F, t46444: F, t766: F, t870: F, t9458: F) -> (F, F, F) {
    let t46445 = t13115 * t9932;
    let t46446 = F::new(36.0) * t46445;
    let t46447 = t32 * t4094;
    let t46449 = F::new(36.0) * t46447 * t2659;
    let t46450 = F::new(6.0) * t1530 * t193 * t870 * t9458 + F::new(3.0) * t193 * t46426 * t766 - t39658 + t46432 - t46434 + t46436 + t46438 + t46439 + t46444 + t46446 + t46449;
    (t46446, t46449, t46450)
}
