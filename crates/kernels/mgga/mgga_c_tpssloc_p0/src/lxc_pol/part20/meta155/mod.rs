//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk987;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk988;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk989;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta155<F: Float>(t182: F, t3681: F, t118: F, t521: F, t2375: F, t1294: F, t2371: F, t2528: F, t1284: F, t172: F, t763: F, t2535: F, t184: F, t17: F, t1388: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3683, t3684) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk987::<F>(t182, t3681, t118, t521);
        let (t3686, t3688, t3690, t3691) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk988::<F>(t2375, t3684, t1294, t2371, t2528, t1284, t172);
        let (t3692, t3693, t3695, t3696) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk989::<F>(t3691, t763, t1294, t2535, t184, t3681);
        let (t3697, t3698) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk990::<F>(t17, t3696, t1388);
    (t3683, t3684, t3686, t3688, t3690, t3691, t3692, t3693, t3695, t3696, t3697, t3698)
}
