//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta585<F: Float>(t20063: F, t3701: F, t1484: F, t2752: F, t17083: F, t225: F, t5584: F, t852: F, t1509: F, t4265: F, t1519: F, t4233: F) -> (F, F, F, F, F, F) {
        let (t57806, t57911, t58143, t58166, t58204, t58226) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1964::<F>(t20063, t3701, t1484, t2752, t17083, t225, t5584, t852, t1509, t4265, t1519, t4233);
    (t57806, t57911, t58143, t58166, t58204, t58226)
}
