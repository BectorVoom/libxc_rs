//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta91 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk630;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk631;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk632;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk633;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk634;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk635;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta91<F: Float>(t118: F, t2393: F, t123: F, t131: F, t2387: F, t2390: F, t693: F, t119: F, t63: F, t133: F, t2388: F, t2391: F, t702: F, t683: F, t681: F, t125: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2394 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk630::<F>(t118, t2393);
        let (t2397, t2398, t2400, t2402) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk631::<F>(t123, t131, t2387, t2390, t693, t119, t63);
        let t2403 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk632::<F>(t133, t2402);
        let t2405 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk633::<F>(t2388, t2391, t2394, t2398, t2400, t2403);
        let (t2406, t2408) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk634::<F>(t2405, t702, t683);
        let t2409 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk635::<F>(t681);
        let (t2410, t2411) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk636::<F>(t2409, t125);
    (t2394, t2397, t2398, t2400, t2402, t2403, t2405, t2406, t2408, t2409, t2410, t2411)
}
