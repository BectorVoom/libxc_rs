//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta681 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2491;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta681<F: Float>(t41115: F, t4250: F, t4166: F, t9637: F, t2649: F, t13257: F, t2617: F, t4184: F, t4257: F, t9993: F, t13176: F, t2638: F, t831: F, t13278: F, t2681: F, t4236: F, t9674: F, t13186: F, t2697: F, t13289: F, t41011: F, t4179: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46649, t46657, t46658, t46661, t46663, t46667) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2491::<F>(t41115, t4250, t4166, t9637, t2649, t13257, t2617, t4184, t4257, t9993, t13176, t2638);
        let (t46668, t46675, t46677, t46679, t46686, t46692) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2492::<F>(t46667, t831, t13278, t2681, t4236, t9674, t13186, t2697, t13289, t41011, t4179, t820);
    (t46649, t46657, t46658, t46661, t46663, t46667, t46668, t46675, t46677, t46679, t46686, t46692)
}
