//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta729 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2390;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2391;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2392;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2393;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2394;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta729<F: Float>(t48157: F, t60192: F, t60194: F, t60202: F, t68571: F, t68577: F, t68580: F, t68583: F, t68586: F, t68589: F, t68592: F, t10564: F, t123: F, t68521: F, t68525: F, t47774: F, t47775: F, t68513: F, t47779: F, t47783: F, t41959: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t60204: F) -> (F, F, F, F, F, F, F) {
        let (t68594, t68596) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2390::<F>(t48157, t60192, t60194, t60202, t68571, t68577, t68580, t68583, t68586, t68589, t68592, t10564, t123, t68521);
        let t68599 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2391::<F>(t10564, t123, t68525);
        let t68602 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2392::<F>(t47774, t47775, t68513);
        let t68605 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2393::<F>(t47774, t47779, t68513);
        let t68608 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2394::<F>(t47774, t47783, t68513);
        let t68616 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2395::<F>(t41959, t59663, t59665, t59680, t59688, t59694, t60204, t68596, t68599, t68602, t68605, t68608);
    (t68594, t68596, t68599, t68602, t68605, t68608, t68616)
}
