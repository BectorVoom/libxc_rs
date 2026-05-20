//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1947;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta661<F: Float>(t23168: F, t28277: F, t28295: F, t6547: F, t6562: F, t7488: F, t86893: F, t28439: F, t28268: F, t81591: F, t17049: F, t1880: F, t6553: F, t6571: F, t1527: F, t776: F, t23270: F, t25038: F, t25191: F, t23204: F, t28294: F, t1493: F, t254: F, t28263: F, t23237: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98921, t98923, t98927, t98932, t98941, t98945) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1947::<F>(t23168, t28277, t28295, t6547, t6562, t7488, t86893, t28439, t28268, t81591, t17049, t1880, t6553, t6571);
        let (t98963, t98966, t98975, t98983, t98986) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1948::<F>(t1527, t776, t23270, t25038, t25191, t23204, t28294, t6562, t1493, t254, t28263, t1880, t23237);
    (t98921, t98923, t98927, t98932, t98941, t98945, t98963, t98966, t98975, t98983, t98986)
}
