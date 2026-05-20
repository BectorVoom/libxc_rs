//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta614<F: Float>(t24571: F, t24574: F, t225: F, t24873: F, t1235: F, t7319: F, t24705: F, t491: F, t24639: F, t24568: F, t24634: F, t3590: F, t7284: F) -> (F, F, F, F, F, F, F, F) {
        let (t85711, t85717, t85724, t85728, t85733, t85739, t85741, t85750) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2055::<F>(t24571, t24574, t225, t24873, t1235, t7319, t24705, t491, t24639, t24568, t24634, t3590, t7284);
    (t85711, t85717, t85724, t85728, t85733, t85739, t85741, t85750)
}
