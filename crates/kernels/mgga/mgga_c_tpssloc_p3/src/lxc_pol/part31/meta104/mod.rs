//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk620;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta104<F: Float>(t730: F, t731: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F, t723: F, t159: F, t167: F, t676: F, t682: F, t268: F, t703: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2461, t2462, t2471, t2472, t2475, t2476, t2477, t2478, t2479, t2480, t2483) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk620::<F>(t730, t731, t2388, t2391, t2394, t2398, t2400, t2403, t723, t159, t167, t676, t682);
        let t2486 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk621::<F>(t2483, t268, t703);
    (t2461, t2462, t2471, t2472, t2475, t2476, t2477, t2478, t2479, t2480, t2483, t2486)
}
