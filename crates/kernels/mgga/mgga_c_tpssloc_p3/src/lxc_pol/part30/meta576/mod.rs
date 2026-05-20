//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1951;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta576<F: Float>(t1484: F, t1649: F, t28: F, t5544: F, t5664: F, t1530: F, t5660: F, t1877: F, t1915: F, t22959: F, t23295: F, t2522: F, t25358: F, t28448: F, t28765: F, t28771: F, t4314: F, t5966: F, t6670: F, t7541: F, t7649: F, t7656: F, t265: F, t504: F, t28755: F, t1409: F, t1972: F, t52: F, t5398: F, t7664: F, t28763: F, t5161: F, t7753: F, t1983: F, t113: F, t1459: F, t1980: F, t24999: F, t27993: F, t27996: F, t28020: F, t28027: F, t28029: F, t28032: F, t28034: F, t28036: F, t28038: F, t28040: F, t28042: F, t28047: F, t28240: F, t510: F, t5460: F, t5494: F, t574: F, t6468: F, t6517: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28774, t28778, t28789, t28792, t28795, t28802) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1951::<F>(t1484, t1649, t28, t5544, t5664, t1530, t5660, t1877, t1915, t22959, t23295, t2522, t25358, t28448, t28765, t28771, t4314, t5966, t6670, t7541, t7649, t7656);
        let (t28803, t28811, t28813, t28816) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1952::<F>(t28, t265, t504, t28755, t1409, t1972, t28802, t52, t5398, t7664, t28763, t5161, t7753, t1983, t113, t1459, t1980, t24999, t27993, t27996, t28020, t28027, t28029, t28032, t28034, t28036, t28038, t28040, t28042, t28047, t28240, t510, t5460, t5494, t574, t6468, t6517, dens_threshold, rho1, zeta_threshold);
    (t28774, t28778, t28789, t28792, t28795, t28803, t28811, t28813, t28816)
}
