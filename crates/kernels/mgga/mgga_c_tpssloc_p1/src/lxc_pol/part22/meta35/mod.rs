//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta35 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk252;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk253;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk254;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk255;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk256;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk257;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk258;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk259;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta35<F: Float>(t676: F, t686: F, t685: F, t118: F, t677: F, t123: F, t67: F, t3: F, t61: F, t119: F, t133: F, t141: F, t683: F, t31: F, t32: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t687, t688, t690) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk252::<F>(t676, t686, t685, t118, t677);
        let (t693, t694, t697) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk253::<F>(t123, t67, t687, t3, t61);
        let t698 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk254::<F>(t119, t697);
        let t699 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk255::<F>(t133, t698);
        let t701 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk256::<F>(t688, t690, t694, t699);
        let t702 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk257::<F>(t141);
        let t703 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk258::<F>(t701, t702);
        let t705 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk259::<F>(t683, t703);
        let t706 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk260::<F>(t31, t32);
    (t688, t690, t693, t694, t697, t698, t699, t701, t702, t703, t705, t706)
}
