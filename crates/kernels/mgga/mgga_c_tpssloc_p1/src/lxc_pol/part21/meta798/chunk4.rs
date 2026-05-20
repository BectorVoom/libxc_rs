//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2776/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2776<F: Float>(t1512: F, t46667: F, t16903: F, t9638: F, t41008: F, t5568: F, t13225: F, t13251: F, t13262: F, t16872: F, t2686: F, t41084: F, t41086: F, t46692: F, t46876: F, t46882: F, t46884: F, t46886: F, t46911: F, t46918: F, t46920: F, t46926: F, t46928: F, t47017: F, t47285: F) -> F {
    let t58731 = t46667 * t1512;
    let t58735 = t9638 * t16903;
    let t58744 = t41008 * t5568;
    let t58754 = -t16872 * t2686 / F::new(3072.0) + F::new(7.0) / F::new(1152.0) * t58731 + F::new(595.0) / F::new(5184.0) * t46876 - F::new(7.0) / F::new(1152.0) * t46882 - F::new(7.0) / F::new(576.0) * t58735 - F::new(7.0) / F::new(2304.0) * t46884 + F::new(119.0) / F::new(3456.0) * t46886 - t13262 * t46692 * t47285 * t47017 / F::new(128.0) + F::new(35.0) / F::new(18.0) * t46911 + F::new(35.0) / F::new(72.0) * t58744 + F::new(455.0) / F::new(324.0) * t41084 - F::new(35.0) / F::new(216.0) * t41086 + t13251 * t13225 / F::new(192.0) + F::new(7.0) / F::new(2304.0) * t46918 - F::new(7.0) / F::new(288.0) * t46920 + F::new(7.0) / F::new(2304.0) * t46926 - F::new(119.0) / F::new(3456.0) * t46928;
    t58754
}
