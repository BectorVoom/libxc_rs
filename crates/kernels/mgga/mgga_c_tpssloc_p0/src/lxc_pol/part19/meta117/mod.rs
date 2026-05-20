//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk640;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk641;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta117<F: Float>(t3237: F, t3238: F, t3245: F, t3250: F, t3254: F, t423: F, t1094: F, t1098: F, t1119: F, t1097: F, t419: F, t409: F, t1117: F, t1118: F, t407: F, t410: F, t1102: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3256, t3258, t3259, t3261, t3262, t3263, t3264) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk640::<F>(t3237, t3238, t3245, t3250, t3254, t423, t1094, t1098, t1119, t1097, t419, t409);
        let t3265 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk641::<F>(t1117);
        let (t3266, t3268, t3270, t3271) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk642::<F>(t1118, t3265, t3264, t407, t410, t1102);
    (t3256, t3258, t3259, t3261, t3262, t3263, t3264, t3265, t3266, t3268, t3270, t3271)
}
