//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2501/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2501<F: Float>(t13316: F, t9638: F, t41115: F, t4240: F, t13278: F, t2686: F, t13173: F, t2639: F, t1512: F, t41340: F, t4236: F, t9671: F) -> (F, F, F, F, F, F) {
    let t46926 = t9638 * t13316;
    let t46928 = t41115 * t4240;
    let t46930 = t13278 * t2686;
    let t46936 = t2639 * t13173;
    let t46951 = t41340 * t1512;
    let t46953 = t9671 * t4236;
    (t46926, t46928, t46930, t46936, t46951, t46953)
}
