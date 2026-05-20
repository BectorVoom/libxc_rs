//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta619<F: Float>(t27495: F, t85821: F, t1193: F, t24811: F, t24660: F, t7319: F, t24667: F, t3545: F, t7372: F, t7378: F, t2121: F, t3427: F, t7381: F) -> (F, F, F, F, F, F, F) {
        let (t85822, t85853, t85859, t85863, t85917, t85918, t85941) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2023::<F>(t27495, t85821, t1193, t24811, t24660, t7319, t24667, t3545, t7372, t7378, t2121, t3427, t7381);
    (t85822, t85853, t85859, t85863, t85917, t85918, t85941)
}
