//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk486;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta74<F: Float>(t40: F, t52: F, t1409: F, t185: F, t707: F, t73: F, t76: F, t145: F, t157: F, t182: F, t767: F, t771: F, zeta_threshold: F, t210: F, t214: F, t785: F, t787: F, t797: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1462, t1464, t1471, t1472, t1473, t1474, t1476, t1484) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk486::<F>(t40, t52, t1409, t185, t707, t73, t76, t145, t157, t182, t767, t771, zeta_threshold);
        let (t1489, t1492) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk487::<F>(t1484, t210, t214, t785, t787, t797);
    (t1462, t1464, t1471, t1472, t1473, t1474, t1476, t1484, t1489, t1492)
}
