//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1787;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1788;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta559<F: Float>(t1519: F, t213: F, t225: F, t23168: F, t25229: F, t794: F, t23164: F, t6555: F, t7480: F, t81632: F, t23030: F, t25035: F, t23228: F, t7479: F, t81573: F, t25059: F, t6562: F, t82082: F, t82087: F, t7488: F, t82133: F, t25225: F, t6547: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86873, t86886, t86893, t86895, t86903, t86911) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1787::<F>(t1519, t213, t225, t23168, t25229, t794, t23164, t6555, t7480, t81632, t23030, t25035);
        let (t86916, t86928, t86930, t86931, t86940, t86942) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1788::<F>(t23228, t7479, t81573, t25059, t6562, t794, t82082, t82087, t7488, t82133, t25225, t6547);
    (t86873, t86886, t86893, t86895, t86903, t86911, t86916, t86928, t86930, t86931, t86940, t86942)
}
