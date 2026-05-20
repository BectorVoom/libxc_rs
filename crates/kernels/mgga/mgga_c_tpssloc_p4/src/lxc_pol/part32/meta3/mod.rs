//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta3 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk19;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk20;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk21;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk22;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk23;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk24;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk25;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk26;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta3<F: Float>(t46: F, rho1: F, sigma2: F, t31: F, sigma0: F, sigma1: F, t3: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47, t48, t50) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk19::<F>(t46, rho1);
        let t51 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk20::<F>(t50, sigma2);
        let t52 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk21::<F>(t31);
        let (t53, t54, t55) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk22::<F>(t52);
        let t56 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk23::<F>(t53, t55);
        let t59 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk24::<F>(sigma0, sigma1, sigma2);
        let t60 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk25::<F>(t3);
        let t61 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk26::<F>(t60);
    (t47, t48, t50, t51, t52, t53, t54, t55, t56, t59, t60, t61)
}
