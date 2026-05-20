//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk536;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk537;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk538;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk539;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk540;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk541;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk542;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk543;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk544;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk545;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta74<F: Float>(t1499: F, t237: F, t1464: F, t1473: F, t1476: F, t225: F, t680: F, t705: F, t752: F, t760: F, t765: F, t1484: F, t824: F, t228: F, t230: F, t232: F, t819: F, t820: F, t847: F, t1496: F, t249: F, t787: F, t803: F, t817: F, t840: F, t843: F, t218: F, t860: F, t235: F, t226: F, t255: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1500, t1504) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk536::<F>(t1499, t237, t1464, t1473, t1476, t225, t680, t705, t752, t760, t765);
        let t1506 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk537::<F>(t1484, t824);
        let t1509 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk538::<F>(t1504, t1506, t228, t230);
        let t1510 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk539::<F>(t1509, t232);
        let t1512 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk540::<F>(t1510, t819, t820);
        let t1516 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk541::<F>(t1484, t820, t847);
        let t1519 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk542::<F>(t1496, t1500, t1512, t1516, t249, t787, t803, t817, t840, t843);
        let (t1520, t1523) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk543::<F>(t1519, t218, t1510, t860);
        let t1525 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk544::<F>(t1519, t235);
        let t1527 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk545::<F>(t1499, t1523, t1525, t226, t255, t812);
    (t1500, t1504, t1506, t1509, t1510, t1512, t1516, t1519, t1520, t1523, t1525, t1527)
}
