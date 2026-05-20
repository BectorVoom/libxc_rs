//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk732;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk733;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk734;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk735;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta143<F: Float>(t2218: F, t2221: F, t2225: F, t2232: F, t1406: F, t604: F, t1437: F, t645: F, t1409: F, t607: F, t25: F, t28: F, t65: F, t2219: F, zeta_threshold: F, t31: F) -> (F, F, F, F, F, F, F) {
        let (t3951, t3953) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk732::<F>(t2218, t2221, t2225, t2232, t1406, t604);
        let (t3958, t3961) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk733::<F>(t1437, t645, t1409, t607);
        let (t3962, t3966) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk734::<F>(t25, t28, t3961, t65, t2219, zeta_threshold);
        let t3967 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk735::<F>(t31, t3966);
    (t3951, t3953, t3958, t3961, t3962, t3966, t3967)
}
