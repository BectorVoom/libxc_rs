//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta724 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2370;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2371;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2372;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2373;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2374;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2375;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2376;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta724<F: Float>(t123: F, t41664: F, t68477: F, t21130: F, t607: F, t2768: F, t68462: F, t68466: F, t68470: F, t21123: F, t690: F, t41684: F, t41863: F, t68460: F, t68464: F, t68468: F, t68472: F, t21127: F) -> (F, F, F, F, F, F, F, F, F) {
        let t68479 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2370::<F>(t123, t41664, t68477);
        let (t68481, t68483) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2371::<F>(t21130, t607, t123, t2768);
        let t68486 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2372::<F>(t123, t2768, t68462);
        let t68489 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2373::<F>(t123, t2768, t68466);
        let t68492 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2374::<F>(t123, t2768, t68470);
        let t68494 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2375::<F>(t21123, t690);
        let t68496 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2376::<F>(t41684, t41863, t68460, t68464, t68468, t68472, t68479, t68483, t68486, t68489, t68492, t68494);
        let t68498 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2377::<F>(t21127, t690);
    (t68479, t68481, t68483, t68486, t68489, t68492, t68494, t68496, t68498)
}
