//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta89 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk618;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk619;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk620;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk621;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk622;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk623;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk624;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta89<F: Float>(t52: F, t636: F, t76: F, t111: F, t649: F, t107: F, t2281: F, t626: F, t667: F, t106: F, t655: F, t94: F, t102: F, t177: F, t738: F, t745: F, t746: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2296, t2298) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk618::<F>(t52, t636, t76);
        let t2314 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk619::<F>(t111, t649);
        let (t2327, t2328, t2331) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk620::<F>(t107, t2281, t626, t667, t106, t655);
        let t2341 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk621::<F>(t94);
        let t2349 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk622::<F>(t102);
        let (t2367, t2368) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk623::<F>(t177, t738);
        let t2369 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk624::<F>(t745);
        let t2371 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk625::<F>(t2368, t2369, t746);
    (t2296, t2298, t2314, t2327, t2328, t2331, t2341, t2349, t2367, t2368, t2369, t2371)
}
