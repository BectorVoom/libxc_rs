//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2280/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2280<F: Float>(t39658: F, t41254: F, t41258: F, t41262: F, t46377: F, t46384: F, t46385: F, t46386: F, t46389: F, t46432: F, t46434: F, t46436: F, t46438: F, t46439: F, t46444: F, t46446: F, t46449: F) -> F {
    let t47148 = t41254 - t46377 - t46384 - t41258 - t46385 - t41262 - t46386 + t46389 + t46432 - t46434 + t46436 + t46438 + t46439 - t39658 + t46444 + t46446 + t46449;
    t47148
}
