//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta113 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk757;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk758;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk759;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk760;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk761;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk762;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta113<F: Float>(t2787: F, t914: F, t287: F, t891: F, t275: F, t912: F, t913: F, t273: F, t276: F, t896: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2789, t2790, t2791) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk757::<F>(t2787, t914, t287, t891);
        let t2792 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk758::<F>(t275, t2791);
        let t2793 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk759::<F>(t912);
        let (t2794, t2796, t2798) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk760::<F>(t2793, t913, t2792, t273, t276);
        let t2799 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk761::<F>(t896);
        let (t2800, t2802, t2807) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk762::<F>(t2798, t2799, t2764, t2766, t2773, t2778, t2782);
    (t2789, t2790, t2791, t2792, t2793, t2794, t2796, t2798, t2799, t2800, t2802, t2807)
}
