//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk955;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta208<F: Float>(t1044: F, t248: F, t4347: F, t1009: F, t1603: F, t1011: F, t1019: F, t1040: F, t1611: F, t4353: F, t4356: F, t4358: F, t4361: F, t4398: F, t4402: F, t4480: F, t4482: F, t4485: F, t4487: F, t4491: F, t4495: F, t4500: F, t360: F, t1021: F, t1020: F, t1025: F, t1041: F, t1046: F, t1618: F, t1622: F, t3104: F, t3109: F, t3114: F, t3117: F, t3140: F, t3156: F, t3160: F, t3163: F, t378: F, t4617: F, t4622: F, t4625: F, t4631: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4636, t4639, t4640, t4641, t4644, t4649) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk955::<F>(t1044, t248, t4347, t1009, t1603, t1011, t1019, t1040, t1611, t4353, t4356, t4358, t4361, t4398, t4402, t4480, t4482, t4485, t4487, t4491, t4495, t4500);
        let (t4650, t4652, t4656) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk956::<F>(t360, t4649, t1021, t248, t1020, t1025, t1041, t1046, t1618, t1622, t3104, t3109, t3114, t3117, t3140, t3156, t3160, t3163, t378, t4617, t4622, t4625, t4631, t4636, t4641, t4644);
    (t4636, t4639, t4640, t4641, t4644, t4649, t4650, t4652, t4656)
}
