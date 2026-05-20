//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta107<F: Float>(t182: F, t2448: F, t676: F, t724: F, t164: F, t723: F, t159: F, t730: F, t731: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F) -> (F, F, F, F, F, F, F) {
        let (t2450, t2454, t2459, t2460, t2461, t2462, t2471) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk637::<F>(t182, t2448, t676, t724, t164, t723, t159, t730, t731, t2388, t2391, t2394, t2398, t2400, t2403);
    (t2450, t2454, t2459, t2460, t2461, t2462, t2471)
}
