//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1270;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1271;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta265<F: Float>(t218: F, t7510: F, t1527: F, t1911: F, t2718: F, t1484: F, t6638: F, t6637: F, t6552: F, t232: F, t4282: F, t6646: F, t1888: F, t1519: F, t1894: F, t214: F, t1880: F, t1510: F, t6657: F, t235: F, t1499: F, t1909: F, t226: F, t6636: F, t6645: F, t812: F, t858: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7511, t7517, t7520, t7521, t7522, t7524, t7525) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1270::<F>(t218, t7510, t1527, t1911, t2718, t1484, t6638, t6637, t6552, t232, t4282, t6646);
        let (t7528, t7529, t7533, t7535, t7537) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1271::<F>(t1888, t7525, t1519, t1894, t214, t1880, t1510, t6657, t235, t7510, t1499, t1909, t226, t6636, t6645, t7522, t812);
        let t7538 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1272::<F>(t7537, t858);
    (t7511, t7517, t7520, t7521, t7524, t7525, t7528, t7529, t7533, t7535, t7537, t7538)
}
