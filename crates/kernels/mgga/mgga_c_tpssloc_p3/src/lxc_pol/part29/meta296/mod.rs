//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta296 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1336;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta296<F: Float>(t181: F, t686: F, t781: F, t756: F, t118: F, t753: F, t2375: F, t2371: F, t677: F, t2374: F, t2535: F, t2528: F) -> (F, F, F, F, F, F, F, F) {
        let (t9874, t9876, t9880, t9882, t9884, t9885, t9887, t9888) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1336::<F>(t181, t686, t781, t756, t118, t753, t2375, t2371, t677, t2374, t2535, t2528);
    (t9874, t9876, t9880, t9882, t9884, t9885, t9887, t9888)
}
