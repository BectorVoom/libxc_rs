//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2269/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2269<F: Float>(t12971: F, t13283: F, t13300: F, t1484: F, t1512: F, t2553: F, t2643: F, t2645: F, t2684: F, t2701: F, t4119: F, t41399: F, t4236: F, t46952: F, t46954: F, t46957: F, t46960: F, t46962: F, t46974: F, t46980: F, t776: F, t820: F, t843: F, t9516: F, t9613: F, t9978: F, t9983: F) -> F {
    let t46982 = -t41399 * t1512 / F::new(3072.0) - t9613 * t4236 / F::new(1024.0) + F::new(5.0) / F::new(768.0) * t843 * t2701 * t820 * t1484 * t9516 - t46952 - t46954 + t13283 * t9983 / F::new(512.0) - t46957 * t9978 / F::new(512.0) + F::new(7.0) / F::new(1536.0) * t46960 - F::new(35.0) / F::new(384.0) * t46962 + F::new(5.0) / F::new(256.0) * t843 * t2701 * t820 * t12971 * t776 + F::new(5.0) / F::new(256.0) * t843 * t2701 * t820 * t4119 * t2553 - F::new(7.0) / F::new(192.0) * t46974 + t2643 * t2645 * t13300 * t2684 / F::new(256.0) - F::new(7.0) / F::new(384.0) * t46980;
    t46982
}
