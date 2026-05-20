//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta508<F: Float>(t25224: F, t6555: F, t6552: F, t1911: F, t4300: F, t2718: F, t1519: F, t828: F, t232: F, t6646: F, t1888: F, t13384: F) -> (F, F, F, F, F, F, F, F) {
        let (t25229, t25230, t25233, t25236, t25237, t25238, t25239, t25241) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1871::<F>(t25224, t6555, t6552, t1911, t4300, t2718, t1519, t828, t232, t6646, t1888, t13384);
    (t25229, t25230, t25233, t25236, t25237, t25238, t25239, t25241)
}
