//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2266/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2266<F: Float>(t41008: F, t4155: F, t13076: F, t9638: F, t13322: F, t13316: F, t41115: F, t4240: F, t13278: F, t2686: F, t13173: F, t2639: F) -> (F, F, F, F, F, F, F) {
    let t46911 = t41008 * t4155;
    let t46912 = F::new(35.0) / F::new(24.0) * t46911;
    let t46918 = t9638 * t13076;
    let t46920 = t9638 * t13322;
    let t46926 = t9638 * t13316;
    let t46928 = t41115 * t4240;
    let t46929 = F::new(119.0) / F::new(4608.0) * t46928;
    let t46930 = t13278 * t2686;
    let t46936 = t2639 * t13173;
    (t46912, t46918, t46920, t46926, t46929, t46930, t46936)
}
