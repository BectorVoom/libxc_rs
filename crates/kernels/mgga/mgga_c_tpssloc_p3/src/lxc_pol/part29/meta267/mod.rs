//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1256;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1257;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta267<F: Float>(t510: F, t7467: F, t652: F, t1484: F, t25: F, t1915: F, t6554: F, t6553: F, t6552: F, t1519: F, t225: F, t258: F, t214: F, t1880: F, t1527: F, t6571: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t7468 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1256::<F>(t510, t7467);
        let (t7470, t7475, t7476, t7479) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1257::<F>(t652, t7468, t1484, t25, t1915, t6554);
        let (t7480, t7481, t7484, t7485, t7486, t7488) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1258::<F>(t6553, t7479, t6552, t1519, t225, t258, t214, t1880, t1527, t6571);
    (t7468, t7470, t7475, t7476, t7479, t7480, t7481, t7484, t7485, t7486, t7488)
}
