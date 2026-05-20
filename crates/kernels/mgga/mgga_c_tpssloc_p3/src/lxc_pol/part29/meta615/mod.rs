//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta615<F: Float>(t24574: F, t24630: F, t24605: F, t85639: F, t24888: F, t24705: F, t7327: F, t1176: F, t1184: F, t24847: F, t974: F, t1009: F, t460: F) -> (F, F, F, F, F, F) {
        let (t85766, t85787, t85789, t85814, t85820, t85821) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2056::<F>(t24574, t24630, t24605, t85639, t24888, t24705, t7327, t1176, t1184, t24847, t974, t1009, t460);
    (t85766, t85787, t85789, t85814, t85820, t85821)
}
