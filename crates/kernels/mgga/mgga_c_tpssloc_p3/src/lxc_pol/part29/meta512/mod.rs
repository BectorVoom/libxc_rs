//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta512<F: Float>(t235: F, t25160: F, t4234: F, t6657: F, t25249: F, t829: F, t6646: F, t22986: F, t22996: F, t4283: F, t1888: F, t1484: F, t23153: F) -> (F, F, F, F, F, F, F, F) {
        let (t25295, t25297, t25299, t25300, t25301, t25303, t25304, t25306) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1877::<F>(t235, t25160, t4234, t6657, t25249, t829, t6646, t22986, t22996, t4283, t1888, t1484, t23153);
    (t25295, t25297, t25299, t25300, t25301, t25303, t25304, t25306)
}
