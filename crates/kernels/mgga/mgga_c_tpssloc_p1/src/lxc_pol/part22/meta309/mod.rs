//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta309<F: Float>(t300: F, t4832: F, t14704: F, t14710: F, t14722: F, t14781: F, t14720: F, t3375: F, t4857: F, t225: F, t4947: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14858, t14868, t14870, t14886, t14890, t14922, t14923, t14924, t14946, t14947, t14960, t14972) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1482::<F>(t300, t4832, t14704, t14710, t14722, t14781, t14720, t3375, t4857, t225, t4947);
    (t14858, t14868, t14870, t14886, t14890, t14922, t14923, t14924, t14946, t14947, t14960, t14972)
}
