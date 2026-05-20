//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2275/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2275<F: Float>(t39373: F, t39397: F, t39400: F, t39408: F, t39411: F, t40685: F, t40689: F, t40708: F, t40714: F, t40716: F, t46143: F, t46144: F, t46152: F, t46194: F, t46195: F, t46197: F, t46207: F) -> F {
    let t47139 = t46143 + t46144 - t40685 + t46152 + t46194 + t46195 + t46197 + t40689 + t39373 - t39397 - t39400 + t40708 + t39408 + t39411 + t46207 - t40714 + t40716;
    t47139
}
