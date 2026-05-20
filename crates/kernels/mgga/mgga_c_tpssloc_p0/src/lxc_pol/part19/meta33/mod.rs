//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta33 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk239;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk240;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta33<F: Float>(t111: F, t89: F, t107: F, t626: F, t106: F, t38: F, t606: F, tau0: F, t109: F, t95: F, t103: F, t100: F, t92: F, t96: F, t64: F, t510: F, t3: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t652, t654, t655, t656, t657, t659) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk239::<F>(t111, t89, t107, t626, t106, t38, t606, tau0);
        let (t660, t662, t666, t667, t671) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk240::<F>(t109, t659, t95, t103, t100, t657, t92, t96, t656, t64, t654);
        let (t672, t676) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk241::<F>(t510, t671, t3, t60);
    (t652, t655, t656, t657, t659, t660, t662, t666, t667, t671, t672, t676)
}
