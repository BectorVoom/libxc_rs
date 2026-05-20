//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta70 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk511;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk512;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk513;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk514;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta70<F: Float>(t1395: F, t3: F, t576: F, t112: F, t577: F, t671: F, t582: F, t586: F, t589: F, t593: F, t596: F, t600: F, t4: F, t581: F, t25: F, t28: F, zeta_threshold: F, t31: F, t65: F, t43: F, t46: F, t48: F, rho1: F, sigma2: F, t55: F, t39: F, t51: F, t56: F, t627: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1396, t1398, t1401) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk511::<F>(t1395, t3, t576, t112);
        let (t1404, t1406, t1408) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk512::<F>(t1395, t1401, t577, t671, t582, t586, t589, t593, t596, t600, t4, t581);
        let t1409 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk513::<F>(t25, t28, t1408, zeta_threshold);
        let (t1410, t1411, t1414, t1420) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk514::<F>(t1409, t31, t65, t43, t46, t48, rho1, sigma2);
        let t1426 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk515::<F>(t1409, t55, t1414, t1420, t39, t51, t56, t627);
    (t1396, t1398, t1401, t1404, t1406, t1408, t1409, t1410, t1411, t1414, t1420, t1426)
}
