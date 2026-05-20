//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2267/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2267<F: Float>(t13186: F, t13242: F, t16836: F, t2623: F, t2643: F, t41084: F, t41086: F, t41088: F, t4167: F, t46912: F, t46918: F, t46920: F, t46926: F, t46929: F, t46930: F, t46936: F, t9634: F, t9646: F, t9647: F, t9663: F) -> F {
    let t46938 = t46912 + F::new(455.0) / F::new(216.0) * t41084 - F::new(35.0) / F::new(72.0) * t41086 + F::new(7.0) / F::new(144.0) * t41088 + t16836 * t9634 / F::new(512.0) + F::new(7.0) / F::new(1536.0) * t46918 - F::new(7.0) / F::new(192.0) * t46920 - F::new(5.0) / F::new(256.0) * t2643 * t9646 * t13242 * t9647 + F::new(7.0) / F::new(1536.0) * t46926 - t46929 + F::new(7.0) / F::new(1536.0) * t46930 - t4167 * t9663 / F::new(3072.0) - F::new(15.0) / F::new(128.0) * t2623 * t13186 + F::new(7.0) / F::new(1536.0) * t46936;
    t46938
}
