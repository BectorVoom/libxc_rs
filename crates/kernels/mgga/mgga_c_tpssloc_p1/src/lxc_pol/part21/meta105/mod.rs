//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta105 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk723;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk724;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk725;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk726;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk727;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk728;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk729;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk730;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta105<F: Float>(t237: F, t2613: F, t68: F, t808: F, t816: F, t809: F, t838: F, t842: F, t233: F, t813: F, t236: F, t240: F, t812: F, t828: F, t232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2614, t2617) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk723::<F>(t237, t2613, t68, t808);
        let t2618 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk724::<F>(t2617, t816);
        let (t2621, t2623) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk725::<F>(t809, t838, t2617, t842);
        let t2627 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk726::<F>(t233, t813);
        let t2628 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk727::<F>(t236, t2627);
        let t2629 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk728::<F>(t240, t2628);
        let (t2630, t2631) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk729::<F>(t2629, t812, t828);
        let t2632 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk730::<F>(t232);
        let t2633 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk731::<F>(t2631, t2632);
    (t2614, t2617, t2618, t2621, t2623, t2627, t2628, t2629, t2630, t2631, t2632, t2633)
}
