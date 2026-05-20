//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta33 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk237;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk238;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk239;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk240;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk241;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk242;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk243;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta33<F: Float>(t676: F, t686: F, t685: F, t118: F, t677: F, t123: F, t67: F, t3: F, t61: F, t119: F, t133: F, t141: F, t683: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t687, t688, t690) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk237::<F>(t676, t686, t685, t118, t677);
        let (t693, t694, t697) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk238::<F>(t123, t67, t687, t3, t61);
        let t698 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk239::<F>(t119, t697);
        let t699 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk240::<F>(t133, t698);
        let t701 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk241::<F>(t688, t690, t694, t699);
        let t702 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk242::<F>(t141);
        let t703 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk243::<F>(t701, t702);
        let t705 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk244::<F>(t683, t703);
    (t688, t690, t693, t694, t697, t698, t699, t701, t702, t703, t705)
}
