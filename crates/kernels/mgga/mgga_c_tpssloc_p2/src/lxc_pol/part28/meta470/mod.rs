//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1679;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta470<F: Float>(t25224: F, t6572: F, t1880: F, t6555: F, t6552: F, t1519: F, t828: F, t232: F, t6646: F, t1888: F, t13384: F, t23110: F, t7524: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25225, t25226, t25229, t25230, t25236, t25237, t25238, t25239, t25241, t25242, t25243, t25245) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1679::<F>(t25224, t6572, t1880, t6555, t6552, t1519, t828, t232, t6646, t1888, t13384, t23110, t7524);
    (t25225, t25226, t25229, t25230, t25236, t25237, t25238, t25239, t25241, t25242, t25243, t25245)
}
