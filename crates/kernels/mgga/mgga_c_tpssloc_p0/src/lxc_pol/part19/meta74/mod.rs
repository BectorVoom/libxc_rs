//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk445;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta74<F: Float>(t94: F, t659: F, t2248: F, t95: F, t102: F, t662: F, t103: F, t100: F, t2336: F, t657: F, t660: F, t92: F, t96: F, t109: F, t656: F, t2327: F, t2328: F, t2333: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2341, t2342, t2343, t2346, t2349, t2350, t2354, t2358) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk445::<F>(t94, t659, t2248, t95, t102, t662, t103, t100, t2336, t657, t660, t92, t96);
        let (t2359, t2363) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk446::<F>(t109, t2358, t656, t2327, t2328, t2333, t64);
    (t2341, t2342, t2343, t2346, t2349, t2350, t2354, t2358, t2359, t2363)
}
