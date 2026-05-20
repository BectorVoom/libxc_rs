//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta331<F: Float>(t3585: F, t820: F, t10401: F, t3575: F, t3610: F, t3624: F, t3521: F, t3579: F, t3577: F, t248: F, t3494: F, t3570: F) -> (F, F, F, F, F, F, F) {
        let (t11668, t11678, t11692, t11697, t11698, t11699, t11702) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1389::<F>(t3585, t820, t10401, t3575, t3610, t3624, t3521, t3579, t3577, t248, t3494, t3570);
    (t11668, t11678, t11692, t11697, t11698, t11699, t11702)
}
