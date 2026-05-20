//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta321<F: Float>(t11647: F, t485: F, t3576: F, t3604: F, t3585: F, t820: F, t10401: F, t3575: F, t3610: F, t3624: F, t3521: F, t1190: F, t3030: F) -> (F, F, F, F, F, F, F) {
        let (t11649, t11665, t11668, t11678, t11692, t11697, t11707) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1345::<F>(t11647, t485, t3576, t3604, t3585, t820, t10401, t3575, t3610, t3624, t3521, t1190, t3030);
    (t11649, t11665, t11668, t11678, t11692, t11697, t11707)
}
