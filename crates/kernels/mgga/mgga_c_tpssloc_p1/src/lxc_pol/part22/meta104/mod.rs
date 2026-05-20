//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta104 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk708;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk709;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk710;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk711;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk712;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta104<F: Float>(t290: F, t2764: F, t919: F, t923: F, t307: F, t922: F, t302: F, t2822: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2843, t2844) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk708::<F>(t290);
        let (t2848, t2856, t2859, t2860) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk709::<F>(t2764, t919, t923, t307, t922);
        let t2861 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk710::<F>(t2860, t302);
        let (t2868, t2875, t2884) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk711::<F>(t2764, t2822, t922);
        let t2885 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk712::<F>(t2884);
        let t2886 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk713::<F>(t2885, t302);
    (t2843, t2844, t2848, t2856, t2859, t2860, t2861, t2868, t2875, t2884, t2885, t2886)
}
