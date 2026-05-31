//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2228/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2228<F: Float>(t12945: F, t2427: F, t12935: F, t193: F, t2522: F, t39400: F, t39408: F, t39411: F, t40708: F, t40714: F, t40716: F, t4119: F, t46207: F, t46209: F, t46213: F, t776: F) -> (F, F) {
    let t46217 = t2427 * t12945;
    let t46218 = F::cast_from(12.0_f64) * t46217;
    let t46219 = F::cast_from(18.0_f64) * t12935 * t193 * t4119 + F::cast_from(9.0_f64) * t2522 * t46213 * t776 - t39400 + t39408 + t39411 + t40708 - t40714 + t40716 + t46207 - t46209 + t46218;
    (t46218, t46219)
}
