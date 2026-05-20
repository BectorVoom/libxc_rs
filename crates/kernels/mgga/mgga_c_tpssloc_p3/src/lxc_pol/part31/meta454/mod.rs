//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta454<F: Float>(t25115: F, t815: F, t6605: F, t23077: F, t6604: F, t4255: F, t841: F, t4234: F, t23083: F, t7500: F, t4159: F, t6581: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25116, t25117, t25119, t25120, t25121, t25123, t25124, t25126, t25128) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1605::<F>(t25115, t815, t6605, t23077, t6604, t4255, t841, t4234, t23083, t7500, t4159, t6581);
    (t25116, t25117, t25119, t25120, t25121, t25123, t25124, t25126, t25128)
}
