//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1131;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta263<F: Float>(t6646: F, t7524: F, t1888: F, t1519: F, t1894: F, t214: F, t1880: F, t1530: F, t25: F, t1484: F, t28: F, t1458: F, t88: F, t1778: F, t191: F, t192: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7525, t7526, t7528, t7529, t7530, t7545, t7649, t7656, t7676) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1131::<F>(t6646, t7524, t1888, t1519, t1894, t214, t1880, t1530, t25, t1484, t28, t1458, t88);
        let (t7684, t7685) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1132::<F>(t1778, t191, t192);
    (t7525, t7526, t7528, t7529, t7530, t7545, t7649, t7656, t7676, t7684, t7685)
}
