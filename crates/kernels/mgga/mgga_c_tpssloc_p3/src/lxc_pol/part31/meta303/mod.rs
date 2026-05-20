//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta303<F: Float>(t1058: F, t10936: F, t3030: F, t990: F, t3032: F, t3129: F, t3038: F, t2775: F, t283: F, t3185: F, t3199: F, t1014: F, t10471: F) -> (F, F, F, F, F, F, F) {
        let (t10937, t10949, t10952, t10969, t11034, t11037, t11045) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1191::<F>(t1058, t10936, t3030, t990, t3032, t3129, t3038, t2775, t283, t3185, t3199, t1014, t10471);
    (t10937, t10949, t10952, t10969, t11034, t11037, t11045)
}
