//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta467<F: Float>(t1512: F, t23041: F, t4166: F, t6613: F, t831: F, t23053: F, t4236: F, t6614: F, t1878: F, t23033: F, t221: F, t4255: F) -> (F, F, F, F, F, F, F) {
        let (t25144, t25146, t25147, t25149, t25151, t25154, t25155) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1676::<F>(t1512, t23041, t4166, t6613, t831, t23053, t4236, t6614, t1878, t23033, t221, t4255);
    (t25144, t25146, t25147, t25149, t25151, t25154, t25155)
}
