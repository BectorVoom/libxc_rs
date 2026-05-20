//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2499;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta685<F: Float>(t2570: F, t2585: F, t4255: F, t46853: F, t13326: F, t9638: F, t2628: F, t2691: F, t4184: F, t812: F, t1512: F, t41362: F, t4166: F, t9666: F, t2635: F, t13337: F, t838: F, t2693: F, t4163: F, t41008: F, t4155: F, t13076: F, t13322: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46855, t46870, t46874, t46876) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2499::<F>(t2570, t2585, t4255, t46853, t13326, t9638, t2628, t2691, t4184, t812, t1512, t41362);
        let (t46882, t46884, t46886, t46911, t46918, t46920) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2500::<F>(t4166, t9666, t2635, t13337, t838, t2693, t4163, t41008, t4155, t13076, t9638, t13322);
    (t46855, t46870, t46874, t46876, t46882, t46884, t46886, t46911, t46918, t46920)
}
