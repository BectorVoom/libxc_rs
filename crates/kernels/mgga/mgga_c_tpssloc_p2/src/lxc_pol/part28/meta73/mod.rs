//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk483;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk484;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk485;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta73<F: Float>(t1408: F, t95: F, t50: F, t103: F, t100: F, t104: F, t92: F, tau1: F, t109: F, t656: F, t64: F, t654: F, t510: F) -> (F, F, F, F, F, F, F, F) {
        let (t1444, t1445, t1447, t1449, t1453) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk483::<F>(t1408, t95, t50, t103, t100, t104, t92, tau1);
        let (t1454, t1458) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk484::<F>(t109, t1453, t656, t64, t654);
        let t1459 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk485::<F>(t1458, t510);
    (t1444, t1445, t1447, t1449, t1453, t1454, t1458, t1459)
}
