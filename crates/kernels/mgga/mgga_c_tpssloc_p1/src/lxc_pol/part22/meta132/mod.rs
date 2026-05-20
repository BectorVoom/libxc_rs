//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta132 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk873;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk874;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta132<F: Float>(t1266: F, t1458: F, t1454: F, t626: F, t1453: F, t2331: F, t666: F, t1444: F, t2341: F, t659: F, t2: F, t95: F, t584: F, t1449: F, t2349: F, t662: F, t103: F, t100: F, t1445: F, t1447: F, t657: F, t663: F, t92: F, t109: F, t656: F, t2327: F, t2328: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4037, t4041, t4043, t4044, t4049, t4050, t4053) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk873::<F>(t1266, t1458, t1454, t626, t1453, t2331, t666, t1444, t2341, t659, t2, t95);
        let (t4059, t4060, t4064, t4067) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk874::<F>(t4053, t584, t1449, t2349, t662, t103, t2, t100, t1445, t1447, t4050, t657, t663, t92);
        let (t4068, t4072) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk875::<F>(t109, t4067, t656, t2327, t2328, t4041, t4044, t64);
    (t4037, t4041, t4043, t4044, t4049, t4059, t4060, t4064, t4067, t4068, t4072)
}
