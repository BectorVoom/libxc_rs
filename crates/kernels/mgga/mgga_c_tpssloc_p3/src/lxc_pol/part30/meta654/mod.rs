//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta654<F: Float>(t1920: F, t2966: F, t7561: F, t225: F, t25789: F, t23384: F, t25802: F, t23587: F, t7560: F, t25410: F, t25798: F, t25822: F) -> (F, F, F, F, F, F, F) {
        let (t89617, t89620, t89630, t89648, t89653, t89662, t89666) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2070::<F>(t1920, t2966, t7561, t225, t25789, t23384, t25802, t23587, t7560, t25410, t25798, t25822);
    (t89617, t89620, t89630, t89648, t89653, t89662, t89666)
}
