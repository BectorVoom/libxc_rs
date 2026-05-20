//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta474<F: Float>(t23164: F, t25316: F, t1519: F, t234: F, t776: F, t6637: F, t6552: F, t1894: F, t4265: F, t214: F, t1880: F, t23237: F, t7479: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25317, t25319, t25320, t25321, t25322, t25324, t25325, t25326, t25338) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1685::<F>(t23164, t25316, t1519, t234, t776, t6637, t6552, t1894, t4265, t214, t1880, t23237, t7479);
    (t25317, t25319, t25320, t25321, t25322, t25324, t25325, t25326, t25338)
}
