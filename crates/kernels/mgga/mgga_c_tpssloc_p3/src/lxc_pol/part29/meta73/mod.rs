//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk498;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk499;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk500;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk501;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta73<F: Float>(t5: F, t1406: F, t1437: F, t605: F, t86: F, t112: F, t1408: F, t95: F, t50: F, t103: F, t100: F, t104: F, t92: F, tau1: F, t109: F, t656: F, t64: F, t654: F, t510: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t1441 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk498::<F>(t5, t1406, t1437, t605, t86);
        let t1442 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk499::<F>(t112, t1441);
        let (t1444, t1445, t1447, t1449, t1453) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk500::<F>(t1408, t95, t50, t103, t100, t104, t92, tau1);
        let (t1454, t1458) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk501::<F>(t109, t1453, t656, t64, t654);
        let t1459 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk502::<F>(t1458, t510);
    (t1441, t1442, t1444, t1445, t1447, t1449, t1453, t1454, t1458, t1459)
}
