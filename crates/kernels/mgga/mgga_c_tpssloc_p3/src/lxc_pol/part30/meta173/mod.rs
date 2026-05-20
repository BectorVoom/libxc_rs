//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk867;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk868;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk869;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta173<F: Float>(t4615: F, t68: F, t369: F, t1031: F, t1611: F, t1036: F, t1612: F, t1616: F, t248: F, t3101: F, t1020: F, t1044: F, t4347: F, t1009: F, t1603: F, t1011: F, t1019: F, t1040: F, t4353: F, t4356: F, t4358: F, t4361: F, t4398: F, t4402: F, t4480: F, t4482: F, t4485: F, t4487: F, t4491: F, t4495: F, t4500: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4616, t4617, t4622, t4625, t4630) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk867::<F>(t4615, t68, t369, t1031, t1611, t1036, t1612, t1616, t248, t3101);
        let (t4631, t4636, t4639, t4640, t4641) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk868::<F>(t1020, t4630, t1044, t248, t4347, t1009, t1603, t1011, t1019);
        let t4644 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk869::<F>(t1040, t1611);
        let t4649 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk870::<F>(t4353, t4356, t4358, t4361, t4398, t4402, t4480, t4482, t4485, t4487, t4491, t4495, t4500);
    (t4616, t4617, t4622, t4625, t4630, t4631, t4636, t4639, t4640, t4641, t4644, t4649)
}
