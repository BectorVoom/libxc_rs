//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1235;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta272<F: Float>(t6553: F, t7479: F, t6552: F, t1519: F, t225: F, t258: F, t214: F, t1880: F, t1527: F, t6571: F, t1492: F, t1902: F, t1496: F, t6581: F, t1484: F, t236: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7480, t7481, t7484, t7485, t7486, t7488) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1235::<F>(t6553, t7479, t6552, t1519, t225, t258, t214, t1880, t1527, t6571);
        let (t7489, t7490, t7492, t7494, t7496) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1236::<F>(t6553, t7488, t1880, t1492, t1902, t1496, t6581, t1484, t236);
    (t7480, t7481, t7484, t7485, t7486, t7488, t7489, t7490, t7492, t7494, t7496)
}
