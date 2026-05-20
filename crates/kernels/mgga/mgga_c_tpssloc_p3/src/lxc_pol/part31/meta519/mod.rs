//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1724;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1725;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1726;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1727;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta519<F: Float>(t2057: F, t5527: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t24344: F, t2522: F, t26744: F, t28248: F, t29105: F, t4314: F, t5544: F, t5660: F, t5664: F, t7114: F, t7845: F, t870: F, t25: F, t265: F, t394: F, t1409: F, t2064: F, t29124: F, t40: F, t5398: F, t7865: F, t28764: F, t1649: F, t24191: F, t28: F, t28771: F, t28774: F, t28778: F, t28789: F, t28792: F, t28795: F, t29106: F, t5966: F, t7649: F, t7656: F, dens_threshold: F, rho0: F, zeta_threshold: F, t504: F, t2071: F, t52: F, t7884: F, t5161: F, t7940: F, t1458: F, t7890: F, rho1: F, t113: F, t1442: F, t1459: F, t1774: F, t1849: F, t1983: F, t2075: F, t2096: F, t27188: F, t28821: F, t28943: F, t28952: F, t28959: F, t28969: F, t4028: F, t510: F, t5450: F, t5457: F, t5460: F, t5494: F, t652: F, t7042: F, t7458: F, t7685: F, t7787: F, t7802: F, t7806: F, t7900: F, t7941: F) -> (F, F, F, F, F, F, F, F) {
        let (t29125, t29148) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1724::<F>(t2057, t5527, t1484, t1530, t1877, t193, t202, t24344, t2522, t26744, t28248, t29105, t4314, t5544, t5660, t5664, t7114, t7845, t870);
        let (t29149, t29156, t29157, t29188) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1725::<F>(t25, t265, t394, t29148, t1409, t2064, t29124, t40, t5398, t7865, t2057, t28764, t1649, t1877, t24191, t24344, t2522, t26744, t28, t28771, t28774, t28778, t28789, t28792, t28795, t29106, t4314, t5966, t7114, t7649, t7656, t7845, dens_threshold, rho0, zeta_threshold);
        let (t29189, t29197, t29201, t29205) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1726::<F>(t28, t265, t504, t29148, t1409, t2071, t29188, t52, t5398, t7884, t29156, t5161, t7940, t1458, t7890, dens_threshold, rho1, zeta_threshold);
        let t29210 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1727::<F>(t113, t1442, t1459, t1774, t1849, t1983, t2075, t2096, t27188, t28821, t28943, t28952, t28959, t28969, t29197, t29201, t29205, t4028, t510, t5450, t5457, t5460, t5494, t652, t7042, t7458, t7685, t7787, t7802, t7806, t7890, t7900, t7941);
    (t29125, t29149, t29157, t29189, t29197, t29201, t29205, t29210)
}
