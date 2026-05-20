//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta417<F: Float>(t16312: F, t550: F, t1339: F, t22827: F, t242: F, t6943: F, t1336: F) -> (F, F, F, F, F) {
        let (t22828, t22829, t22830, t22832, t22833) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1590::<F>(t16312, t550, t1339, t22827, t242, t6943, t1336);
    (t22828, t22829, t22830, t22832, t22833)
}
