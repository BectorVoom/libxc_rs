//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk643;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk644;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk645;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta93<F: Float>(t607: F, t751: F, t707: F, t195: F, t197: F, t676: F, t724: F, t164: F, t723: F, t159: F, t730: F, t731: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2430, t2431, t2433) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk643::<F>(t607, t751, t707, t195);
        let t2440 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk644::<F>(t197);
        let (t2454, t2458, t2459, t2460, t2461) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk645::<F>(t676, t724, t164, t723, t159, t730);
        let (t2462, t2471) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk646::<F>(t2461, t731, t2388, t2391, t2394, t2398, t2400, t2403);
    (t2430, t2431, t2433, t2440, t2454, t2458, t2459, t2460, t2461, t2462, t2471)
}
