//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta306<F: Float>(t11203: F, t1114: F, t2403: F, t241: F, t3439: F, t407: F, t11135: F, t410: F, t417: F, t1097: F, t3311: F, t409: F) -> (F, F, F, F, F, F, F) {
        let (t11204, t11211, t11219, t11243, t11247, t11265, t11275) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1195::<F>(t11203, t1114, t2403, t241, t3439, t407, t11135, t410, t417, t1097, t3311, t409);
    (t11204, t11211, t11219, t11243, t11247, t11265, t11275)
}
