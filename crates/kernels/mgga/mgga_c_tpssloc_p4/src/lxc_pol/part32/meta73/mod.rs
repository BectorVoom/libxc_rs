//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk488;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk489;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk490;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk491;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk492;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk493;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta73<F: Float>(t1409: F, t31: F, t65: F, t43: F, t46: F, t48: F, rho1: F, sigma2: F, t55: F, t39: F, t51: F, t56: F, t627: F, t33: F, t634: F, t638: F, t72: F, t66: F, t80: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1410 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk488::<F>(t1409, t31);
        let (t1411, t1414, t1417, t1419) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk489::<F>(t1410, t65, t1409, t43, t46, t48, rho1);
        let t1420 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk490::<F>(t1419, sigma2);
        let (t1423, t1426) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk491::<F>(t1409, t55, t1414, t1420, t39, t51, t56, t627);
        let (t1427, t1433) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk492::<F>(t1426, t33, t1409, t634, t638);
        let t1434 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk493::<F>(t1433, t72);
        let t1437 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk494::<F>(t1411, t1427, t1434, t66, t80);
    (t1410, t1411, t1414, t1417, t1419, t1420, t1423, t1426, t1427, t1433, t1434, t1437)
}
