//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta34 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk249;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk250;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk251;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk252;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk253;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk254;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk255;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk256;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta34<F: Float>(t38: F, tau0: F, t606: F, t95: F, t103: F, t100: F, t92: F, t96: F, t109: F, t656: F, t64: F, t654: F, t510: F, t3: F, t60: F, t120: F, t118: F, t142: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t657 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk249::<F>(t38, tau0);
        let t659 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk250::<F>(t606);
        let (t660, t662) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk251::<F>(t659, t95);
        let (t663, t666) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk252::<F>(t103, t662, t100, t657, t660, t92, t96);
        let (t667, t671) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk253::<F>(t109, t656, t666, t64, t654);
        let t672 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk254::<F>(t510, t671);
        let t676 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk255::<F>(t3, t60);
        let t677 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk256::<F>(t120, t676);
        let t680 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk257::<F>(t118, t142, t677);
    (t657, t659, t660, t662, t663, t666, t667, t671, t672, t676, t677, t680)
}
